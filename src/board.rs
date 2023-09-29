use rogalik_math::{
    aabb::Aabb,
    vectors::{Vector2f, Vector2i}
};
use rogalik_engine::Color;
use std::collections::HashSet;

use crate::globals::{TILE_SIZE, BOARD_WIDTH};
use crate::sprite::StaticSprite;

#[derive(Default)]
pub struct Board {
    pub colliders: Vec<Aabb>,
    pub sprites: Vec<StaticSprite>,
    pub gates: Vec<Gate>
}

pub struct Gate {
    pub position: Vector2f,
    pub has_passenger: bool,
    pub since_pickup: f32
}
impl Gate {
    pub fn clear_passenger(&mut self) {
        self.since_pickup = 0.;
        self.has_passenger = false;
    }
}

pub fn generate_board() -> Board 
{
    let mut sprites = Vec::new();
    let mut colliders = Vec::new();

    let mut rocks = HashSet::new();

    for x in 0..BOARD_WIDTH {
        rocks.insert(Vector2i::new(x as i32 - BOARD_WIDTH as i32 / 2, - 1));
    }
    rocks.insert(Vector2i::new(2, 2));
    rocks.insert(Vector2i::new(3, 2));
    rocks.insert(Vector2i::new(4, 2));
    rocks.insert(Vector2i::new(-4, 5));
    rocks.insert(Vector2i::new(-3, 5));
    
    for r in rocks.iter() {
        let (sprite, aabb) = get_rock(*r, &rocks);
        sprites.push(sprite);
        colliders.push(aabb);
    }

    let gate_pos = vec![
        Vector2f::new(TILE_SIZE * 2., TILE_SIZE * 3.),
        Vector2f::new(TILE_SIZE * -3., TILE_SIZE * 6.),
        Vector2f::new(-TILE_SIZE * 4., 0.)
    ];
    let mut gates = Vec::new();
    for (i, g) in gate_pos.iter().enumerate() {
        let (sprite, gate) = get_gate(*g, i as u32);
        sprites.push(sprite);
        gates.push(gate);
    }

    Board { sprites, colliders, gates }
}


fn get_rock(v: Vector2i, other: &HashSet<Vector2i>) -> (StaticSprite, Aabb) {
    let position = v.as_f32() * TILE_SIZE;
    let mut offset = 0;
    if !other.contains(&(v + Vector2i::UP)) { offset += 1 };
    if !other.contains(&(v + Vector2i::DOWN)) { offset += 2 };
    let sprite = StaticSprite {
        atlas: "tiles",
        index: offset,
        color: Color(255, 255, 255, 255),
        size: Vector2f::new(TILE_SIZE, TILE_SIZE),
        position
    };
    let aabb = Aabb::new(position, position + Vector2f::new(TILE_SIZE, TILE_SIZE));
    (sprite, aabb)
}

fn get_gate(position: Vector2f, number: u32) -> (StaticSprite, Gate) {
    let sprite = StaticSprite {
        atlas: "tiles",
        index: 4,
        color: Color(255, 255, 255, 255),
        size: Vector2f::new(TILE_SIZE, TILE_SIZE),
        position
    };
    let gate = Gate { position, has_passenger: false, since_pickup: 0. };
    (sprite, gate)
}