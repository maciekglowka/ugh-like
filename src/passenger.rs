use rogalik_engine::Color;
use rogalik_math::{
    aabb::Aabb,
    vectors::Vector2f
};

use super::State;
use crate::globals::{PASSENGER_LOAD_DIST, PASSENGER_WALK_SPEED, TOLERANCE, TILE_SIZE};
use crate::player::Player;
use crate::sprite::DynamicSprite;
use crate::utils::almost_eq;

pub enum PassengerState {
    Waiting(f32),
    Landed(Vector2f)
}

pub struct Passenger {
    pub sprite: DynamicSprite,
    pub state: PassengerState,
    pub target_gate: u32
}
impl Passenger {
    pub fn new(
        position: Vector2f,
        atlas: &'static str,
        sprite_index: usize,
        color: Color,
        collider_size: Vector2f,
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
            target_gate
        }
    }
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
        state.player.passenger = Some(passenger);
    }
}

pub fn try_unload(state: &mut State) {
    let gate_no = if let Some(passenger) = &state.player.passenger {
        passenger.target_gate
    } else {
        return;
    };
    if state.player.v.len() > TOLERANCE { return }
    let Some(&gate) = state.board.gates.get(gate_no as usize) else { return };
    if (state.player.sprite.centre() - gate_centre(gate)).len() > PASSENGER_LOAD_DIST {
        return
    }

    let mut passenger = state.player.passenger.take().unwrap();
    passenger.state = PassengerState::Landed(gate);
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