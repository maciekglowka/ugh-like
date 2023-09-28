use rand::prelude::*;
use rogalik_engine::Color;
use rogalik_math::{
    aabb::Aabb,
    vectors::Vector2f
};

use super::State;
use crate::board::Gate;
use crate::globals::{
    PASSENGER_LOAD_DIST, PASSENGER_WALK_SPEED, TOLERANCE, TILE_SIZE, SPAWN_TICK,
    PASSENGER_WIDTH, PASSENGER_HEIGHT
};
use crate::player::Player;
use crate::sprite::DynamicSprite;
use crate::utils::almost_eq;

#[derive(PartialEq)]
pub enum PassengerState {
    Waiting(f32),
    Landed(Vector2f)
}

pub struct Passenger {
    pub sprite: DynamicSprite,
    pub state: PassengerState,
    pub source_gate: u32,
    pub target_gate: u32
}
impl Passenger {
    pub fn new(
        position: Vector2f,
        atlas: &'static str,
        sprite_index: usize,
        color: Color,
        collider_size: Vector2f,
        source_gate: u32,
        target_gate: u32
    ) -> Self {
        let sprite = DynamicSprite::new(
            position,
            atlas,
            sprite_index,
            color,
            collider_size
        );
        Self {
            sprite,
            state: PassengerState::Waiting(0.),
            target_gate,
            source_gate
        }
    }
}

pub fn try_spawn(state: &mut State) {
    state.since_spawn += SPAWN_TICK;
    for gate in state.board.gates.iter_mut() {
        gate.since_pickup += SPAWN_TICK;
    }

    if state.since_spawn < state.spawn_interval { return }

    let source_candidates = state.board.gates.iter()
        .enumerate()
        .filter(|(_, a)| !a.has_passenger && a.since_pickup > state.spawn_interval)
        .map(|(i, _)| i);

    let mut rng = thread_rng();
    let Some(gate_idx) = source_candidates.choose(&mut rng) else { return };
    let target_candidates = state.board.gates.iter()
        .enumerate()
        .filter(|(i, _)| *i != gate_idx)
        .map(|(i, _)| i);

    let Some(target_gate) = target_candidates.choose(&mut rng) else { return };
    
    if state.board.gates[gate_idx].has_passenger { return };

    let gate_position = state.board.gates[gate_idx].position;

    let passenger = Passenger::new(
        gate_position + Vector2f::new(0.5 * TILE_SIZE, 0.),
        "actors",
        4,
        Color(255, 255, 255, 255),
        Vector2f::new(PASSENGER_WIDTH, PASSENGER_HEIGHT),
        gate_idx as u32,
        target_gate as u32
    );
    state.passengers.push(passenger);
    state.since_spawn = 0.;
    state.board.gates[gate_idx].has_passenger = true;
}

pub fn should_remove(passenger: &Passenger) -> bool {
    match passenger.state {
        PassengerState::Waiting(_) => false,
        PassengerState::Landed(gate) => {
            almost_eq(gate_centre(gate).x, passenger.sprite.centre().x)
        }
    }
}

pub fn move_passenger(passenger: &mut Passenger, player: &Player, delta: f32) {
    let Some(d) = get_walk(passenger, player) else { return };
    let vx = delta * PASSENGER_WALK_SPEED * d.normalized().x;
    passenger.sprite.position.x += vx.clamp(-d.x.abs(), d.x.abs());
}

fn get_walk(passenger: &Passenger, player: &Player) -> Option<Vector2f> {
    match passenger.state {
        PassengerState::Waiting(_) => {
            if should_approach_loading(passenger, player) {
                return Some(player.sprite.centre() - passenger.sprite.centre());
            };
        },
        PassengerState::Landed(gate) => {
            return Some(gate_centre(gate) - passenger.sprite.centre())
        }
    }
    None
}

fn should_approach_loading(passenger: &Passenger, player: &Player) -> bool {
    if player.passenger.is_some() { return false }
    if !same_level(passenger, &player.sprite.position) { return false }

    if (
        passenger.sprite.centre() - player.sprite.centre()
    ).len() > PASSENGER_LOAD_DIST {
        return false
    }

    true
}

pub fn try_load(state: &mut State) {
    if !state.player.grounded { return }
    if state.player.passenger.is_some() { return }

    let mut loaded = None;
    for (i, passenger) in state.passengers.iter().enumerate() {
        if !same_level(passenger, &state.player.sprite.position) { continue; }
        if (passenger.sprite.centre() - state.player.sprite.centre()).len()
            > 0.25 * passenger.sprite.collider_size.x {
            continue;
        }
        loaded = Some(i);
        break;
    }
    if let Some(loaded) = loaded {
        let passenger = state.passengers.remove(loaded);
        state.board.gates[passenger.source_gate as usize].pickup();
        state.player.passenger = Some(passenger);
    }
}

pub fn try_unload(state: &mut State) {
    let gate_no = if let Some(passenger) = &state.player.passenger {
        passenger.target_gate
    } else {
        return;
    };
    if !state.player.grounded { return }
    if state.player.v.len() > TOLERANCE { return }
    let gate_position = if let Some(gate) =  state.board.gates.get(gate_no as usize) {
        gate.position
    } else { 
        return
    };
    if (state.player.sprite.centre() - gate_centre(gate_position)).len() > PASSENGER_LOAD_DIST {
        return
    }

    let mut passenger = state.player.passenger.take().unwrap();
    passenger.state = PassengerState::Landed(gate_position);
    passenger.sprite.position = state.player.sprite.position;
    state.passengers.push(passenger);
}

fn same_level(passenger: &Passenger, v: &Vector2f) -> bool {
    almost_eq(
        passenger.sprite.position.y,
        v.y
    ) 
}
fn gate_centre(position: Vector2f) -> Vector2f {
    position + 0.5 * Vector2f::new(TILE_SIZE, TILE_SIZE)
}