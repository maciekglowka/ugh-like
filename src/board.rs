use rogalik_math::{
    aabb::Aabb,
    vectors::{Vector2f, Vector2i}
};
use rogalik_engine::Color;
use std::collections::{HashMap, HashSet};

use crate::globals::{TILE_SIZE, BOARD_WIDTH, BOARD_HEIGHT};
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

pub fn generate_board(data: &str) -> Board 
{
    let locations = parse_str_data(data);
    let mut sprites = Vec::new();
    let mut colliders = Vec::new();
    
    for r in locations["rocks"].iter() {
        let (sprite, aabb) = get_rock(*r, &locations["rocks"]);
        sprites.push(sprite);
        colliders.push(aabb);
    }
    let mut gates = Vec::new();
    for (i, g) in locations["gates"].iter().enumerate() {
        let (sprite, gate) = get_gate(g.as_f32(), i as u32);
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

fn parse_str_data(data: &str) -> HashMap<&str, HashSet<Vector2i>> {
    // the data should not have multibyte characters
    // so it's safe byte len = char len
    let lines = data.split('\n')
        .map(|s| match s.len() {
            a if a > BOARD_WIDTH as usize => s[..BOARD_WIDTH as usize].to_string(),
            _ => s.to_string()
        })
        .collect::<Vec<_>>();
    if lines.len() != 10 { panic!("Incorrect level data: row count mismatch!")};

    let mut locations = HashMap::from_iter(vec![
        ("rocks", HashSet::new()),
        ("gates", HashSet::new()) 
    ]);

    for (row, line) in lines.iter().enumerate() {
        let y = BOARD_HEIGHT - row as u32 - 1;
        for (col, c) in line.chars().enumerate() {
            let v = Vector2i::new(col as i32, y as i32);
            match c {
                '#' => { locations.get_mut("rocks").unwrap().insert(v); },
                'G' => { locations.get_mut("gates").unwrap().insert(v); },
                _ => ()
            };
        }
    }
    locations
}