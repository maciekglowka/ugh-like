use rogalik_engine::Color;
use rogalik_math::{
    aabb::Aabb,
    vectors::Vector2f
};

use crate::globals::TILE_SIZE;

#[derive(Default)]
pub struct DynamicSprite {
    pub atlas: &'static str,
    pub index: usize,
    pub color: Color,
    pub frame: usize,
    pub position: Vector2f,
    pub collider_size: Vector2f,
    pub sprite_size: Vector2f
}
impl DynamicSprite {
    pub fn new(
        position: Vector2f,
        atlas: &'static str,
        index: usize,
        color: Color,
        collider_size: Vector2f
    ) -> Self {
        Self {
            atlas,
            index,
            position,
            color,
            frame: 0,
            sprite_size: Vector2f::new(TILE_SIZE, TILE_SIZE),
            collider_size
        }
    }
    pub fn aabb(&self) -> Aabb {
        Aabb::new(self.position, self.position + self.collider_size)
    }
    pub fn aabb_moved(&self, offset: Vector2f) -> Aabb {
        let target = self.position + offset;
        Aabb::new(target, target + self.collider_size)
    }
}