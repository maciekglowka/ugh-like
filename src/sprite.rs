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
    pub z_index: i32,
    pub collider_size: Vector2f,
    pub collider_offset: Vector2f,
    pub size: Vector2f,
    pub flip_x: bool
}
impl DynamicSprite {
    pub fn new(
        position: Vector2f,
        z_index: i32,
        atlas: &'static str,
        index: usize,
        color: Color,
        collider_size: Vector2f,
        collider_offset: Vector2f,
    ) -> Self {
        Self {
            atlas,
            index,
            position,
            z_index,
            color,
            frame: 0,
            size: Vector2f::new(TILE_SIZE, TILE_SIZE),
            collider_size,
            collider_offset,
            flip_x: false
        }
    }
    pub fn aabb(&self) -> Aabb {
        let pos = self.position + self.collider_offset;
        Aabb::new(pos, pos + self.collider_size)
    }
    pub fn aabb_moved(&self, offset: Vector2f) -> Aabb {
        let target = self.position + self.collider_offset + offset;
        Aabb::new(target, target + self.collider_size)
    }
    pub fn centre(&self) -> Vector2f {
        // returns collider's centre
        self.position + self.collider_offset + 0.5 * self.collider_size
    }
}

pub struct StaticSprite {
    pub atlas: &'static str,
    pub index: usize,
    pub color: Color,
    pub position: Vector2f,
    pub z_index: i32,
    pub size: Vector2f
}