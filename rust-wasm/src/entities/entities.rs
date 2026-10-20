use crate::core::material::MaterialId;

pub struct EntityData {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum EntityId {
    Rabbit,

    Count,
}

pub struct EntitySprite {
    width: usize,
    height: usize,
    grid: Vec<Vec<MaterialId>>,
}

pub struct EntityProps {
    pub sprite: EntitySprite,
}

/*
pub static ENTITY_PROPS_OF: [EntityProps; EntityId::Count as usize] = [

]
*/
