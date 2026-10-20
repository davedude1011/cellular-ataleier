use strum_macros::IntoStaticStr;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Eq, Debug, IntoStaticStr)]
#[repr(u8)]
pub enum MaterialId {
    Empty,
    Barrier,

    Sand,
    WetSand,

    Water,
    Oil,

    Steam,
    Smog,

    Fire,

    AssetPlantGreen1,
    AssetPlantGreen2,
    AssetPlantGreen3,
    AssetPlantGreen4,

    AssetPlantYellow1,

    AssetPlantBrown1,
    AssetPlantBrown2,

    DebugRay,

    UserCursor,

    Count,
}
pub const MATERIAL_COUNT: usize = MaterialId::Count as usize;

impl MaterialId {
    pub fn is_empty(&self) -> bool {
        *self == MaterialId::Empty
    }

    pub fn is_barrier(&self) -> bool {
        *self == MaterialId::Barrier
    }

    pub fn is_asset(&self) -> bool {
        PROPS_OF[*self as usize].phase == MaterialPhase::Asset
    }

    pub fn is_cellular(&self) -> bool {
        if self.is_empty() || self.is_barrier() {
            return false;
        }

        let phase = PROPS_OF[*self as usize].phase;
        if phase == MaterialPhase::Asset {
            return false;
        }

        true
    }

    pub fn from_color(color: u32) -> Self {
        let idx = PROPS_OF.iter().position(|p| p.color == color).unwrap();
        unsafe { std::mem::transmute(idx as u8) }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, IntoStaticStr)]
pub enum MaterialPhase {
    Empty,
    Immovable,

    Solid,
    Powder,
    Liquid,
    Gas,

    Asset,
    Debug,
    User,
}
impl MaterialPhase {}

pub struct MaterialProps {
    pub phase: MaterialPhase,
    pub color: u32,
    pub density: u8,
    pub dispersion: u8,
}
impl MaterialProps {
    // produces a probability for a specific cell to swap
    // it takes the average density of the first 10 cells above it
    // and uses that to fall-off the default probability for the
    // materials phase, if the average density above it is much higher
    // than its own, then the probability of moving should be much less
    // this relationship should be exponential
    pub fn swap_probability(&self, upper_density: u8) -> f32 {
        let phase_probability = match self.phase {
            MaterialPhase::Empty => 1.0,
            MaterialPhase::Immovable => 0.0,
            MaterialPhase::Solid => 0.0,
            MaterialPhase::Powder => 0.00001,
            MaterialPhase::Liquid => 0.8,
            MaterialPhase::Gas => 1.0,
            _ => 0.0,
        };

        if phase_probability == 1.0 {
            return 1.0;
        }

        if upper_density <= self.density {
            return phase_probability;
        }

        let delta_density = upper_density - self.density;

        const DECAY_RATE: f32 = 0.2;
        let fallaff = (-DECAY_RATE * delta_density as f32).exp();

        let prob = phase_probability * fallaff;
        if prob < 0.0001 {
            return 0.0;
        }

        prob
    }
}
const fn props(color: u32, density: u8, dispersion: u8, phase: MaterialPhase) -> MaterialProps {
    MaterialProps {
        phase,
        color,
        density,
        dispersion,
    }
}

#[rustfmt::skip]
pub static PROPS_OF: [MaterialProps; MATERIAL_COUNT] = [
    /* empty                */ props(0xffffffff, 0,   0, MaterialPhase::Empty),
    /* barrier              */ props(0xff000000, 255, 0, MaterialPhase::Immovable),

    /* sand                 */ props(0xff6ab8d2, 180, 1, MaterialPhase::Powder),
    /* wet sand             */ props(0xff497f91, 200, 0, MaterialPhase::Powder),

    /* water                */ props(0xff91691e, 100, 5, MaterialPhase::Liquid),
    /* oil                  */ props(0xff151e28, 80,  3, MaterialPhase::Liquid),

    /* Steam                */ props(0xffababab, 10, 10, MaterialPhase::Gas),
    /* Smog                 */ props(0xff52666e, 20, 8,  MaterialPhase::Gas),

    /* fire                 */ props(0xff0b42e6, 0,   0, MaterialPhase::Solid),

    /* asset plant green 1  */ props(0xff639c30, 200, 0, MaterialPhase::Asset),
    /* asset plant green 2  */ props(0xff3fc451, 200, 0, MaterialPhase::Asset),
    /* asset plant green 3  */ props(0xff45d689, 200, 0, MaterialPhase::Asset),
    /* asset plant green 4  */ props(0xff45d6b4, 200, 0, MaterialPhase::Asset),

    /* asset plant yellow 1 */ props(0xff60f9fc, 200, 0, MaterialPhase::Asset),
    
    /* asset plant brown 1  */ props(0xff3badff, 200, 0, MaterialPhase::Asset),
    /* asset plant brown 2  */ props(0xff1980c5, 200, 0, MaterialPhase::Asset),

    /* asset debug ray      */ props(0xffe34b49, 255, 0, MaterialPhase::Debug),

    /* user cursor          */ props(0xff100000, 255, 0, MaterialPhase::User),
];
