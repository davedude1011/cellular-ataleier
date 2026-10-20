use std::fmt::format;

use crate::{
    console_log,
    core::{
        interaction::{self, INTERACTION_OF, TRANSMUTATION_OF},
        material::{MaterialId, MaterialPhase, PROPS_OF},
        rng::normal_hash3,
    },
    entities::{
        assets::sunflower::init_entity_sunflower,
        entity::{EntityAnchored, EntityLimb},
    },
};

pub struct Sim {
    pub width: usize,
    pub height: usize,
    pub len: usize,

    mask: usize,
    shift: usize,

    pub entities: Vec<EntityAnchored>,
    pub buffer_material: Box<[MaterialId]>,
    pub material_hash: Box<[(usize, (usize, usize))]>,

    frame: u64,
}

impl Sim {
    pub fn new(width: usize, height: usize) -> Self {
        assert!(
            width.is_power_of_two(),
            "width of canvas must be a multiple of 2"
        );
        assert!(
            height.is_power_of_two(),
            "height of canvas must be a multiple of 2"
        );

        let len = width * height;

        let mask = width - 1;
        let shift = width.ilog2() as usize;

        let buffer_material = vec![MaterialId::Empty; len].into_boxed_slice();
        let material_hash = vec![(0, (0, 0)); MaterialId::Count as usize].into_boxed_slice();

        let mut sim = Sim {
            width,
            height,
            len,
            mask,
            shift,
            entities: vec![],
            buffer_material,
            material_hash,
            frame: 0,
        };

        sim.init_barrier();

        /*
                let e1 = init_entity_sunflower(100, height - 10);
                let e2 = init_entity_sunflower(135, height - 10);
                let e3 = init_entity_sunflower(300, height - 10);

                sim.push_entity(e1);
                sim.push_entity(e2);
                sim.push_entity(e3);
        */

        sim
    }

    pub fn reset_material_hash(&mut self) {
        self.material_hash = vec![(0, (0, 0)); MaterialId::Count as usize].into_boxed_slice();
    }

    pub fn step(&mut self) {
        self.step_entities();
        self.reset_material_hash();

        for idx in (0..self.len).rev() {
            let mat = self.buffer_material[idx];
            if !mat.is_cellular() {
                continue;
            };

            let props = &PROPS_OF[mat as usize];

            // material step
            match props.phase {
                MaterialPhase::Powder => self.displace_attemp_powder(idx),
                MaterialPhase::Liquid => {
                    self.displace_attemp_liquid(idx, props.dispersion as usize)
                }
                MaterialPhase::Gas => self.displace_attempt_gas(idx, props.dispersion as usize),
                _ => {}
            }

            // interactions
            'interactions: {
                let idx_d = idx + self.width;
                let mat_d = self.buffer_material[idx_d];
                if self.attempt_interact(idx, idx_d, mat, mat_d) {
                    break 'interactions;
                };

                let idx_u = idx - self.width;
                let mat_u = self.buffer_material[idx_u];
                if self.attempt_interact(idx, idx_u, mat, mat_u) {
                    break 'interactions;
                };

                let idx_l = idx - 1;
                let mat_l = self.buffer_material[idx_l];
                if self.attempt_interact(idx, idx_l, mat, mat_l) {
                    break 'interactions;
                };

                let idx_r = idx + 1;
                let mat_r = self.buffer_material[idx_r];
                if self.attempt_interact(idx, idx_r, mat, mat_r) {
                    break 'interactions;
                };
            }

            // transmutations
            self.attempt_transmutation(idx, mat);
        }

        self.frame += 1;
    }
}

impl Sim {
    pub fn set_mat(&mut self, idx: usize, material: MaterialId) {
        self.buffer_material[idx] = material;
    }

    fn init_barrier(&mut self) {
        for x in 0..self.width {
            let top = self.idx(x, 0);
            let bottom = self.idx(x, self.height - 1);

            self.set_mat(top, MaterialId::Barrier);
            self.set_mat(bottom, MaterialId::Barrier);
        }

        for y in 0..self.height {
            let left = self.idx(0, y);
            let right = self.idx(self.width - 1, y);

            self.set_mat(left, MaterialId::Barrier);
            self.set_mat(right, MaterialId::Barrier);
        }
    }

    pub fn draw(&mut self, x: usize, y: usize, material: MaterialId) {
        let idx = self.idx(x, y);
        self.set_mat(idx, material);
    }
}

impl Sim {
    pub fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn xy(&self, idx: usize) -> (usize, usize) {
        let x = idx & self.mask;
        let y = idx >> self.shift;

        (x, y)
    }

    fn attempt_interact(
        &mut self,
        idx_a: usize,
        idx_b: usize,
        mat_a: MaterialId,
        mat_b: MaterialId,
    ) -> bool {
        if let Some(interaction) = INTERACTION_OF[mat_a as usize][mat_b as usize] {
            let rng = normal_hash3(self.frame as u32, idx_a as u32, idx_b as u32);
            if rng < interaction.probability {
                self.set_mat(idx_a, interaction.a2);
                self.set_mat(idx_b, interaction.b2);

                return true;
            }
        }

        false
    }

    fn attempt_transmutation(&mut self, idx: usize, mat: MaterialId) -> bool {
        if let Some(transmutation) = TRANSMUTATION_OF[mat as usize] {
            let rng = normal_hash3(self.frame as u32, idx as u32, mat as u32);
            if rng < transmutation.probability {
                self.set_mat(idx, transmutation.a2);

                return true;
            }
        }

        false
    }

    fn avg_upper_density(&self, idx: usize, up: usize) -> u8 {
        let mut total_density = 0;
        let mut counter = 0;
        for dy in 1..=up {
            let didx = dy * self.width;
            if didx > idx {
                break;
            }

            let nidx = idx - didx;
            let mat = self.buffer_material[nidx];
            let prop = &PROPS_OF[mat as usize];

            // casted to avoid total_density being a u8, which leads to "attempted to add with
            // overflow errors"
            total_density += prop.density as u32;
            counter += 1;
        }

        (total_density as f32 / counter as f32) as u8
    }

    fn can_swap(&mut self, a: usize, b: usize) -> bool {
        let mat_a = self.buffer_material[a];
        let mat_b = self.buffer_material[b];

        let props_a = &PROPS_OF[mat_a as usize];
        let props_b = &PROPS_OF[mat_b as usize];

        if props_a.density <= props_b.density {
            return false;
        }

        let avg_upper_density = self.avg_upper_density(b, 10);
        let swap_prob = props_b.swap_probability(avg_upper_density);
        if swap_prob == 0.0 {
            return false;
        }
        if swap_prob == 1.0 {
            return true;
        }

        let rng = normal_hash3(self.frame as u32, a as u32, b as u32);
        if rng > swap_prob {
            return false;
        }

        true
    }

    fn attempt_swap(&mut self, a: usize, b: usize) -> bool {
        if !self.can_swap(a, b) {
            return false;
        }

        let mat_a = self.buffer_material[a];
        self.buffer_material[a] = self.buffer_material[b];
        self.buffer_material[b] = mat_a;

        true
    }

    fn attempt_probabilistic_swap(&mut self, a: usize, b: usize, probability: f32) -> bool {
        if !self.can_swap(a, b) {
            let mat_a = self.buffer_material[a];
            let mat_b = self.buffer_material[b];

            if !mat_a.is_cellular() || !mat_b.is_cellular() {
                return false;
            }

            let rng = normal_hash3(
                (self.frame & 0xffffffff00000000) as u32,
                (self.frame & 0x00000000ffffffff) as u32,
                b as u32,
            );
            if rng > probability {
                return false;
            }
        }

        let mat_a = self.buffer_material[a];
        self.buffer_material[a] = self.buffer_material[b];
        self.buffer_material[b] = mat_a;

        true
    }

    pub fn avg_material_position(&self, material: MaterialId) -> Option<(usize, usize)> {
        let (count, (tx, ty)) = self.material_hash[material as usize];
        if count == 0 {
            return None;
        }

        let ax = tx / count;
        let ay = ty / count;

        Some((ax, ay))
    }
}

/*
* displace attempt macros
*/
macro_rules! displace {
    ($self:ident, left, $x:ident, $y:ident, $idx:ident, $dispersion:ident) => {
        for dx in 1..=$dispersion {
            if dx > $x {
                break;
            }

            let dl = $self.idx($x - dx, $y + 1);
            if $self.attempt_swap($idx, dl) {
                break;
            }

            let l = $idx - dx;
            if $self.attempt_probabilistic_swap($idx, l, 0.01) {
                return;
            }
        }
    };

    ($self:ident, right, $x:ident, $y:ident, $idx:ident, $dispersion:ident) => {
        for dx in 1..=$dispersion {
            if $x + dx >= $self.width {
                break;
            }

            let dr = $self.idx($x + dx, $y + 1);
            if $self.attempt_swap($idx, dr) {
                break;
            }

            let r = $idx + dx;
            if $self.attempt_probabilistic_swap($idx, r, 0.01) {
                return;
            }
        }
    };

    ($self:ident, down_left, $x:ident, $y:ident, $idx:ident) => {
        let idx_dl = $self.idx($x - 1, $y + 1);
        if $self.attempt_swap($idx, idx_dl) {
            return;
        }
    };

    ($self:ident, down_right, $x:ident, $y:ident, $idx:ident) => {
        let idx_dr = $self.idx($x + 1, $y + 1);
        if $self.attempt_swap($idx, idx_dr) {
            return;
        }
    };

    ($self:ident, up_left, $x:ident, $y:ident, $idx:ident) => {
        let idx_ul = $self.idx($x - 1, $y - 1);
        if $self.attempt_swap($idx, idx_ul) {
            return;
        }
    };

    ($self:ident, up_right, $x:ident, $y:ident, $idx:ident) => {
        let idx_ur = $self.idx($x + 1, $y - 1);
        if $self.attempt_swap($idx, idx_ur) {
            return;
        }
    };
}

impl Sim {
    fn displace_attemp_powder(&mut self, idx: usize) {
        let (x, y) = self.xy(idx);

        let idx_d = self.idx(x, y + 1);
        if self.attempt_swap(idx, idx_d) {
            return;
        }

        let rng = normal_hash3(self.frame as u32, x as u32, y as u32);

        if rng > 0.5 {
            displace!(self, down_left, x, y, idx);
        } else {
            displace!(self, down_right, x, y, idx);
        }
    }

    fn displace_attemp_liquid(&mut self, idx: usize, dispersion: usize) {
        let (x, y) = self.xy(idx);

        let idx_d = self.idx(x, y + 1);
        if self.attempt_probabilistic_swap(idx, idx_d, 0.01) {
            return;
        }

        let rng = normal_hash3(self.frame as u32, x as u32, y as u32);

        if rng > 0.56 {
            displace!(self, down_left, x, y, idx);
            displace!(self, left, x, y, idx, dispersion);
        } else {
            displace!(self, down_right, x, y, idx);
            displace!(self, right, x, y, idx, dispersion);
        }
    }

    fn displace_attempt_gas(&mut self, idx: usize, dispersion: usize) {
        let (x, y) = self.xy(idx);

        let idx_u = self.idx(x, y - 1);
        if self.attempt_probabilistic_swap(idx, idx_u, 0.01) {
            return;
        }

        let rng = normal_hash3(self.frame as u32, x as u32, y as u32);

        if rng > 0.56 {
            displace!(self, up_left, x, y, idx);
            displace!(self, left, x, y, idx, dispersion);
        } else {
            displace!(self, up_right, x, y, idx);
            displace!(self, right, x, y, idx, dispersion);
        }
    }
}

impl Sim {
    fn push_entity(&mut self, entity: EntityAnchored) {
        self.entities.push(entity);
    }

    fn step_entities(&mut self) {
        for idx in 0..self.entities.len() {
            let entity = &mut self.entities[idx];
            if !entity.is_dirty() {
                continue;
            }

            entity.clean();

            self.peel_entity(idx);
            self.stamp_entity(idx);
        }
    }

    // gives an iterator over the limbs asset
    // in the form of (local index, global index, maderial at local index)
    fn iter_limb_asset(
        limb: &EntityLimb,
        width: usize,
    ) -> impl Iterator<Item = (usize, usize, MaterialId)> {
        let asset = limb.asset();
        let x0y0 = limb.x0y0();

        (0..asset.height).flat_map(move |ly| {
            let gy = x0y0.1 + ly;

            let local_row_offset = ly * asset.width;
            let global_row_offset = gy * width;

            (0..asset.width).map(move |lx| {
                let gx = x0y0.0 + lx;

                let lidx = local_row_offset + lx;
                let gidx = global_row_offset + gx;

                let local_material = limb.asset_at_xy(lx, ly);

                (lidx, gidx, local_material)
            })
        })
    }

    fn stamp_entity(&mut self, entity_idx: usize) {
        let entity = &self.entities[entity_idx];
        for limb in &entity.limbs {
            if !limb.in_bounds(self.width, self.height) {
                continue;
            }

            for (_, gidx, local_material) in Self::iter_limb_asset(limb, self.width) {
                if local_material.is_empty() {
                    continue;
                }

                self.buffer_material[gidx] = local_material;
            }
        }
    }

    fn peel_entity(&mut self, entity_idx: usize) {
        let entity = &self.entities[entity_idx];
        for limb in &entity.limbs {
            if !limb.in_bounds(self.width, self.height) {
                continue;
            }

            for (_, gidx, _) in Self::iter_limb_asset(limb, self.width) {
                self.buffer_material[gidx] = MaterialId::Empty;
            }
        }
    }
}
