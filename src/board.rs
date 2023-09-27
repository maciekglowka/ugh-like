use rogalik_math::{
    aabb::Aabb,
    vectors::Vector2f
};

use crate::globals::{TILE_SIZE, BOARD_WIDTH};

pub struct Rock {
    pub position: Vector2f,
    pub aabb: Aabb
}
impl Rock {
    pub fn new(position: Vector2f) -> Self {
        Self {
            position,
            aabb: Aabb::new(position, position + Vector2f::new(TILE_SIZE, TILE_SIZE))
        }
    }
}

pub fn generate_board() -> Vec<Rock> {
    let mut rocks = Vec::new();

    for x in 0..BOARD_WIDTH {
        rocks.push(Vector2f::new(TILE_SIZE * (x as f32 - BOARD_WIDTH as f32 / 2.), -TILE_SIZE));
    }
    rocks.push(Vector2f::new(TILE_SIZE * 2., TILE_SIZE * 2.));
    rocks.push(Vector2f::new(TILE_SIZE * 3., TILE_SIZE * 2.));
    rocks.push(Vector2f::new(TILE_SIZE * 4., TILE_SIZE * 2.));
    rocks.push(Vector2f::new(TILE_SIZE * -4., TILE_SIZE * 5.));
    rocks.push(Vector2f::new(TILE_SIZE * -3., TILE_SIZE * 5.));
    rocks.iter().map(|&v| Rock::new(v)).collect()
}