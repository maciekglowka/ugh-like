use rogalik_engine::{Color, GraphicsContext, Params2d};
use rogalik_math::vectors::Vector2f;

use super::{Context_, State};
use crate::globals::{TILE_SIZE, PIXEL_SCALE, BASE_REPUTATION, PASSENGER_MAX_WAIT};
use crate::passenger::PassengerState;
use crate::utils::to_roman;

const UI_BG: Color = Color(85, 113, 119, 255);
const UI_RED: Color = Color(152, 77, 77, 255);

pub fn render_game_ui(state: &State, context: &mut Context_) {
    render_passenger_targets(state, context);
    render_status_bar(state, context);
}

fn render_passenger_targets(state: &State, context: &mut Context_) {
    for passenger in state.passengers.iter() {
        if let PassengerState::Waiting(time) = passenger.state {
            let color = if time > 0.5 * PASSENGER_MAX_WAIT {
                UI_RED
            } else {
                UI_BG
            };
            context.graphics.draw_atlas_sprite(
                state.textures["ascii"],
                219,
                passenger.sprite.centre() + Vector2f::new(- 0.4 * TILE_SIZE, 0.8 * TILE_SIZE),
                Vector2f::new(0.8 * TILE_SIZE, 0.5 * TILE_SIZE),
                Params2d { color, ..Default::default() }
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
}

fn render_status_bar(state: &State, context: &mut Context_) {
    let top = context.get_viewport_size().y / PIXEL_SCALE;
    let base = Vector2f::new(0.25, top - 0.5 * TILE_SIZE);
    let stamina_width = 4. * TILE_SIZE;
    let height = 0.25 * TILE_SIZE;
    let margin = 0.25 * TILE_SIZE;
    let mut offset = 0.;

    // draw reputation

    for i in 0..state.player.stats.reputation {
        context.graphics.draw_atlas_sprite(
            state.textures["ascii"],
            3,
            base + Vector2f::new(offset + i as f32 * height, 0.),
            Vector2f::new(height, height),
            Params2d { color: UI_RED, ..Default::default() }
        );
    }
    offset += BASE_REPUTATION as f32 * height + margin;

    // draw stamina bar
    context.graphics.draw_atlas_sprite(
        state.textures["ascii"],
        219,
        base + Vector2f::new(offset, 0.),
        Vector2f::new(stamina_width, height),
        Params2d { color: UI_BG, ..Default::default() }
    );
    context.graphics.draw_atlas_sprite(
        state.textures["ascii"],
        219,
        base + Vector2f::new(offset, 0.),
        Vector2f::new(state.player.stats.stamina * stamina_width, height),
        Params2d { color: UI_RED, ..Default::default() }
    );
    offset += stamina_width + margin;

    // draw score
    context.graphics.draw_text(
        state.font,
        &format!("Score: {}", state.player.stats.score),
        base + Vector2f::new(offset, 0.),
        height,
        Params2d { color: UI_BG, ..Default::default() }
    );
}

pub fn render_game_over(state: &State, context: &mut Context_) {
    let vs = context.get_viewport_size() / PIXEL_SCALE;
    let centre = Vector2f::new(
        0.5 * vs.x,
        0.5 * vs.y,
    );
    render_centered_text(centre, "GAME OVER", TILE_SIZE, state, context);
    render_centered_text(
        centre - Vector2f::new(0., TILE_SIZE * 1.25),
        &format!("Passengers delivered: {}", state.player.stats.score),
        0.5 *TILE_SIZE,
        state,
        context
    );
    render_centered_text(
        centre - Vector2f::new(0., TILE_SIZE * 2.),
        "(press spacebar)",
        0.5 *TILE_SIZE,
        state,
        context
    );
}

fn text_width(t: &str, height: f32) -> f32 {
    // assuming only single byte text
    height * t.len() as f32
}

fn render_centered_text(v: Vector2f, t: &str, height: f32, state: &State, context: &mut Context_) {
    context.graphics.draw_text(
        state.font,
        &t,
        v - Vector2f::new(0.5 * text_width(&t, height), 0.),
        height,
        Params2d { color: UI_BG, ..Default::default() }
    );
}