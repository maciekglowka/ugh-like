use rogalik_engine::Color;
use rogalik_math::{
    aabb::Aabb,
    vectors::Vector2f
};

use crate::globals::BIRD_SPEED;
use crate::sprite::DynamicSprite;

pub struct Creature {
    pub sprite: DynamicSprite
}
impl Creature {
    pub fn new(
        position: Vector2f,
        atlas: &'static str,
        sprite_index: usize,
        color: Color,
        collider_size: Vector2f,
    ) -> Self {
        let sprite = DynamicSprite::new(
            position,
            atlas,
            sprite_index,
            color,
            collider_size,
            Vector2f::ZERO
        );
        Self {
            sprite
        }
    }
}
