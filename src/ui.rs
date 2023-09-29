use rogalik_engine::{Color, GraphicsContext, Params2d};
use rogalik_math::vectors::Vector2f;

use super::{Context_, State};
use crate::globals::TILE_SIZE;
use crate::passenger::PassengerState;
use crate::utils::to_roman;

pub fn render_ui(state: &State, context: &mut Context_) {
    render_passenger_targets(state, context);
}

fn render_passenger_targets(state: &State, context: &mut Context_) {
    for passenger in state.passengers.iter() {
        match passenger.state {
            PassengerState::Waiting(_) => (),
            _ => continue
        };
        context.graphics.draw_atlas_sprite(
            state.textures["ascii"],
            219,
            passenger.sprite.centre() + Vector2f::new(- 0.4 * TILE_SIZE, 0.8 * TILE_SIZE),
            Vector2f::new(0.8 * TILE_SIZE, 0.5 * TILE_SIZE),
            Params2d { color: Color(64, 85, 89, 255), ..Default::default() }
        );
        let t = to_roman(passenger.target_gate + 1);
        let dx = 0.5 * t.len() as f32 * 0.25 * TILE_SIZE;
        context.graphics.draw_text(
            state.font,
            t,
            passenger.sprite.centre() + Vector2f::new(-dx, 0.9 * TILE_SIZE),
            0.25 * TILE_SIZE,
            Params2d { color: Color(0, 0, 0, 255), ..Default::default() }
        );
    }
}
