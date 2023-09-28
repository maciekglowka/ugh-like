use rogalik_engine::{Context, GraphicsContext, Params2d};
use super::{State, WgpuContext};

use crate::sprite::{DynamicSprite, StaticSprite};

pub fn render_sprites(state: &State, context: &mut Context<WgpuContext>) {
    for sprite in state.board.sprites.iter() {
        render_static_sprite(sprite, state, context);
    }
    render_dynamic_sprite(&state.player.sprite, state, context);
    for passenger in state.passengers.iter() {
        render_dynamic_sprite(&passenger.sprite, state, context);
    }
}

fn render_dynamic_sprite(
    sprite: &DynamicSprite,
    state: &State,
    context: &mut Context<WgpuContext>
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
    context: &mut Context<WgpuContext>
) {
    context.graphics.draw_atlas_sprite(
        state.textures[sprite.atlas],
        sprite.index,
        sprite.position,
        sprite.size,
        Params2d { color: sprite.color, ..Default::default() }
    );
}