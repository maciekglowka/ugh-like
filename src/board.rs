use rogalik_math::{
    aabb::Aabb,
    vectors::Vector2f
};
use rogalik_engine::Color;

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
    pub fn pickup(&mut self) {
        self.since_pickup = 0.;
        self.has_passenger = false;
    }
}

pub fn generate_board() -> Board 
{
    let mut sprites = Vec::new();
    let mut colliders = Vec::new();

    let mut rocks = Vec::new();

    for x in 0..BOARD_WIDTH {
        rocks.push(Vector2f::new(TILE_SIZE * (x as f32 - BOARD_WIDTH as f32 / 2.), -TILE_SIZE));
    }
    rocks.push(Vector2f::new(TILE_SIZE * 2., TILE_SIZE * 2.));
    rocks.push(Vector2f::new(TILE_SIZE * 3., TILE_SIZE * 2.));
    rocks.push(Vector2f::new(TILE_SIZE * 4., TILE_SIZE * 2.));
    rocks.push(Vector2f::new(TILE_SIZE * -4., TILE_SIZE * 5.));
    rocks.push(Vector2f::new(TILE_SIZE * -3., TILE_SIZE * 5.));
    
    for r in rocks {
        let (sprite, aabb) = get_rock(r);
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


fn get_rock(position: Vector2f) -> (StaticSprite, Aabb) {
    let sprite = StaticSprite {
        atlas: "ascii",
        index: 177,
        color: Color(32, 96, 32, 255),
        size: Vector2f::new(TILE_SIZE, TILE_SIZE),
        position
    };
    let aabb = Aabb::new(position, position + Vector2f::new(TILE_SIZE, TILE_SIZE));
    (sprite, aabb)
}

fn get_gate(position: Vector2f, number: u32) -> (StaticSprite, Gate) {
    let sprite = StaticSprite {
        atlas: "ascii",
        index: 177,
        color: Color(96, 32, 96, 255),
        size: Vector2f::new(TILE_SIZE, TILE_SIZE),
        position
    };
    let gate = Gate { position, has_passenger: false, since_pickup: 0. };
    (sprite, gate)
}