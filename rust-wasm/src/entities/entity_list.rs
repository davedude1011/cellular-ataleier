use crate::entities::entities::EntitySprite;

pub struct Entity {
    sprite: &'static EntitySprite,

    x: usize,
    y: usize,
}
