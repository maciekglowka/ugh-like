use rogalik_engine::{Context, GraphicsContext, ResourceId, Params2d, Color};
use rogalik_math::vectors::Vector2f;
use super::{State, WgpuContext};

use crate::player::Player;
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
    render_player(&state.player, state, context);
}

fn render_player(player: &Player, state: &State, context: &mut Context<WgpuContext>) {
    context.graphics.draw_atlas_sprite(
        state.textures[player.atlas],
        player.sprite_index + player.frame,
        player.position,
        Vector2f::new(TILE_SIZE, TILE_SIZE),
        Params2d { color: player.color, ..Default::default() }
    );
}