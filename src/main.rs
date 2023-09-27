use rogalik_engine::{Context, GraphicsContext, Engine, Game, ResourceId, Params2d, Color};
use rogalik_math::vectors::Vector2f;
use rogalik_wgpu::WgpuContext;
use std::collections::HashMap;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

mod actor;
mod board;
mod globals;
mod render;

#[derive(Default)]
pub struct State {
    rocks: Vec<board::Rock>,
    animation_timer: ResourceId,
    textures: HashMap<&'static str, ResourceId>,
    font: ResourceId,
    player: actor::Actor
}
impl Game<WgpuContext> for State {
    fn setup(&mut self, context: &mut Context<WgpuContext>) {
        load_assets(self, context);
        self.rocks = board::generate_board();
        self.player = actor::Actor::new(Vector2f::new(0., 2.), "actors", 0, Color(255, 255, 255, 255));
    }
    fn update(&mut self, context: &mut Context<WgpuContext>) {
        if context.input.is_key_down(rogalik_engine::input::VirtualKeyCode::W) {
            self.player.v.y = globals::LIFT_SPEED;
        } else {
            self.player.v.y = 0.;
        }

        self.player.v.x = 0.;
        if context.input.is_key_down(rogalik_engine::input::VirtualKeyCode::D) {
            self.player.v.x = globals::FLY_SPEED;
            // self.flip = false;
        }
        if context.input.is_key_down(rogalik_engine::input::VirtualKeyCode::A) {
            self.player.v.x = -globals::FLY_SPEED;
            // self.flip = false;
        }

        if context.time.get_timer(self.animation_timer).unwrap().is_finished() {
            if self.player.v.y > 0. {
                self.player.frame += 1;
                self.player.frame = self.player.frame % globals::ACTOR_FRAMES;
            }
        }

        let obstacles = self.rocks.iter().map(|a| a.aabb).collect();
        actor::move_actor(&mut self.player, &obstacles, context.time.get_delta());
        render::render_sprites(self, context);
    }
}

fn main() {
    std::env::set_var("WINIT_UNIX_BACKEND", "x11");
    run();
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
fn run() {
    let state = State::default();
    let engine = Engine::new(state);
    engine.run();
}

fn load_assets(state: &mut State, context: &mut Context<WgpuContext>) {
    let sprite_bytes_ascii = include_bytes!("../assets/ascii.png");
    let sprite_bytes_actors = include_bytes!("../assets/actors.png");
    let sprite_bytes_kenney = include_bytes!("../assets/kenney.png");

    let t_0 = context.graphics.load_sprite_atlas(
        sprite_bytes_ascii,
        16,
        16
    );
    let t_1 = context.graphics.load_sprite_atlas(
        sprite_bytes_actors,
        4,
        4
    );
    let t_2 = context.graphics.load_sprite_atlas(
        sprite_bytes_kenney,
        20,
        20
    );
    state.textures.insert("ascii", t_0);
    state.textures.insert("actors", t_1);
    state.textures.insert("kenney", t_2);
    state.font = context.graphics.load_font(
        sprite_bytes_ascii,
        16,
        16
    );

    let camera_0 = context.graphics.create_camera(64.0, Vector2f::new(0., 4.));
    context.graphics.set_camera(camera_0);

    state.animation_timer = context.time.add_timer(globals::ANIMATION_TICK);
    context.graphics.set_clear_color(Color(0, 6, 12, 255));
}