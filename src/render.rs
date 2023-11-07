use rogalik_engine::{GraphicsContext, Params2d, Color};
use rogalik_math::vectors::Vector2f;
use super::{State, Context_};

use crate::globals::{
    TILE_SIZE, BOARD_HEIGHT, BOARD_WIDTH, BACKGROUND_Z, UI_Z
};
use crate::sprite::{DynamicSprite, StaticSprite};
use crate::utils::to_roman;

pub fn render_sprites(state: &State, context: &mut Context_) {
    render_background(state, context);
    for sprite in state.board.sprites.iter() {
        render_static_sprite(sprite, state, context);
    }
    render_gate_numbers(state, context);
    render_dynamic_sprite(&state.player.sprite, state, context);
    for passenger in state.passengers.iter() {
        render_dynamic_sprite(&passenger.sprite, state, context);
    }
    for creature in state.creatures.iter() {
        render_dynamic_sprite(&creature.sprite, state, context);
    }
}

fn render_background(state: &State, context: &mut Context_) {
    let base = Vector2f::new(0., 0.);
    for x in 0..BOARD_WIDTH {
        for y in 0..BOARD_HEIGHT {
            context.graphics.draw_atlas_sprite(
                "tiles",
                (8 + ((x + y) % 3)) as usize,
                (base + Vector2f::new(x as f32, y as f32)) * TILE_SIZE,
                BACKGROUND_Z,
                Vector2f::new(TILE_SIZE, TILE_SIZE),
                Params2d::default()
            );
        }
    }
}

fn render_dynamic_sprite(
    sprite: &DynamicSprite,
    state: &State,
    context: &mut Context_
) {
    context.graphics.draw_atlas_sprite(
        sprite.atlas,
        sprite.index + sprite.frame,
        sprite.position,
        sprite.z_index,
        sprite.size,
        Params2d { color: sprite.color, flip_x: sprite.flip_x, ..Default::default() }
    );
}

fn render_static_sprite(
    sprite: &StaticSprite,
    state: &State,
    context: &mut Context_
) {
    context.graphics.draw_atlas_sprite(
        sprite.atlas,
        sprite.index,
        sprite.position,
        sprite.z_index,
        sprite.size,
        Params2d { color: sprite.color, ..Default::default() }
    );
}

fn render_gate_numbers(state: &State, context: &mut Context_) {
    for (i, gate) in state.board.gates.iter().enumerate() {
        let t = to_roman(i as u32 + 1);
        let dx = 0.45 * t.len() as f32 * 0.25;
        context.graphics.draw_text(
            "default",
            t,
            gate.position + Vector2f::new(TILE_SIZE * (0.5 - dx), 1.04 * TILE_SIZE),
            UI_Z,
            0.25 * TILE_SIZE,
            Params2d { color: Color(64, 85, 89, 255), ..Default::default()}
        );
    }
}