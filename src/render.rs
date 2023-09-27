use rogalik_engine::{Context, GraphicsContext, ResourceId, Params2d, Color};
use rogalik_math::vectors::Vector2f;
use super::{State, WgpuContext};

use crate::actor::Actor;
use crate::globals::TILE_SIZE;

pub fn render_sprites(state: &State, context: &mut Context<WgpuContext>) {
    for rock in state.rocks.iter() {
        context.graphics.draw_atlas_sprite(
            state.textures["ascii"],
            177,
            rock.position,
            Vector2f::new(TILE_SIZE, TILE_SIZE),
            Params2d { color: Color(32, 96, 32, 255), ..Default::default() }
        );
    }
    render_actor(&state.player, state, context);
}

fn render_actor(actor: &Actor, state: &State, context: &mut Context<WgpuContext>) {
    context.graphics.draw_atlas_sprite(
        state.textures[actor.atlas],
        actor.sprite_index + actor.frame,
        actor.position,
        Vector2f::new(TILE_SIZE, TILE_SIZE),
        Params2d { color: actor.color, ..Default::default() }
    );
}