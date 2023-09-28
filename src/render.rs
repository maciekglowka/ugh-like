use rogalik_engine::{GraphicsContext, Params2d};
use super::{State, Context_};

use crate::globals::TILE_SIZE;
use crate::sprite::{DynamicSprite, StaticSprite};
use crate::utils::to_roman;

pub fn render_sprites(state: &State, context: &mut Context_) {
    for sprite in state.board.sprites.iter() {
        render_static_sprite(sprite, state, context);
    }
    render_gate_numbers(state, context);
    render_dynamic_sprite(&state.player.sprite, state, context);
    for passenger in state.passengers.iter() {
        render_dynamic_sprite(&passenger.sprite, state, context);
    }
}

fn render_dynamic_sprite(
    sprite: &DynamicSprite,
    state: &State,
    context: &mut Context_
) {
    context.graphics.draw_atlas_sprite(
        state.textures[sprite.atlas],
        sprite.index + sprite.frame,
        sprite.position,
        sprite.size,
        Params2d { color: sprite.color, ..Default::default() }
    );
}

fn render_static_sprite(
    sprite: &StaticSprite,
    state: &State,
    context: &mut Context_
) {
    context.graphics.draw_atlas_sprite(
        state.textures[sprite.atlas],
        sprite.index,
        sprite.position,
        sprite.size,
        Params2d { color: sprite.color, ..Default::default() }
    );
}

fn render_gate_numbers(state: &State, context: &mut Context_) {
    for (i, gate) in state.board.gates.iter().enumerate() {
        context.graphics.draw_text(
            state.font,
            to_roman(i as u32 + 1),
            gate.position,
            0.25 * TILE_SIZE,
            Params2d::default()
        );
    }
}