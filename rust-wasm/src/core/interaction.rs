use crate::core::material::{MATERIAL_COUNT, MaterialId};

#[derive(Clone, Copy)]
pub struct MaterialInteraction {
    pub a1: MaterialId,
    pub b1: MaterialId,

    pub a2: MaterialId,
    pub b2: MaterialId,

    pub probability: f32,
}
impl MaterialInteraction {
    pub const fn new(
        a1: MaterialId,
        b1: MaterialId,
        a2: MaterialId,
        b2: MaterialId,
        probability: f32,
    ) -> Self {
        Self {
            a1,
            b1,
            a2,
            b2,
            probability,
        }
    }
}

pub static INTERACTION_OF: [[Option<MaterialInteraction>; MATERIAL_COUNT]; MATERIAL_COUNT] = {
    let mut interaction_table = [[None; MATERIAL_COUNT]; MATERIAL_COUNT];

    macro_rules! insert_interaction {
        (singular, $a1:expr, $b1:expr, $a2:expr, $b2:expr, $probability:expr) => {{
            let interaction = MaterialInteraction::new($a1, $b1, $a2, $b2, $probability);
            interaction_table[interaction.a1 as usize][interaction.b1 as usize] = Some(interaction);
        }};
        (bidirectional, $a1:expr, $b1:expr, $a2:expr, $b2:expr, $probability:expr) => {{
            let interaction = MaterialInteraction::new($a1, $b1, $a2, $b2, $probability);
            interaction_table[interaction.a1 as usize][interaction.b1 as usize] = Some(interaction);
            interaction_table[interaction.b1 as usize][interaction.a1 as usize] = Some(interaction);
        }};
    }

    insert_interaction!(
        singular,
        MaterialId::Water,
        MaterialId::Sand,
        MaterialId::Empty,
        MaterialId::WetSand,
        0.1
    );

    insert_interaction!(
        singular,
        MaterialId::Fire,
        MaterialId::Oil,
        MaterialId::Fire,
        MaterialId::Fire,
        0.2
    );
    insert_interaction!(
        singular,
        MaterialId::Oil,
        MaterialId::Fire,
        MaterialId::Smog,
        MaterialId::Fire,
        0.1
    );

    insert_interaction!(
        bidirectional,
        MaterialId::Fire,
        MaterialId::Water,
        MaterialId::Steam,
        MaterialId::Empty,
        0.9
    );

    interaction_table
};

#[derive(Clone, Copy)]
pub struct MaterialTransmutation {
    pub a1: MaterialId,
    pub a2: MaterialId,

    pub probability: f32,
}
impl MaterialTransmutation {
    pub const fn new(a1: MaterialId, a2: MaterialId, probability: f32) -> Self {
        Self {
            a1,
            a2,
            probability,
        }
    }
}

pub static TRANSMUTATION_OF: [Option<MaterialTransmutation>; MATERIAL_COUNT] = {
    let mut transmutation_table = [None; MATERIAL_COUNT];

    macro_rules! insert_interaction {
        ($a1:expr, $a2:expr, $probability:expr) => {{
            let interaction = MaterialTransmutation::new($a1, $a2, $probability);
            transmutation_table[interaction.a1 as usize] = Some(interaction);
        }};
    }

    insert_interaction!(MaterialId::Fire, MaterialId::Empty, 0.05);

    transmutation_table
};
