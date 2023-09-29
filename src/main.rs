use rogalik_engine::{Context, GraphicsContext, Engine, Game, ResourceId, Color};
use rogalik_math::vectors::Vector2f;
use rogalik_wgpu::WgpuContext;
use std::collections::{HashMap, VecDeque};

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

type Context_ = Context<WgpuContext>;

mod board;
mod globals;
mod passenger;
mod player;
mod render;
mod sprite;
mod ui;
mod utils;

#[derive(Default)]
pub struct State {
    board: board::Board,
    animation_timer: ResourceId,
    textures: HashMap<&'static str, ResourceId>,
    font: ResourceId,
    player: player::Player,
    passengers: Vec<passenger::Passenger>,
    since_spawn: f32,
    spawn_timer: ResourceId,
    spawn_interval: f32
}
impl Game<WgpuContext> for State {
    fn setup(&mut self, context: &mut Context_) {
        load_assets(self, context);
        self.board = board::generate_board();
        self.player = player::Player::new(
            Vector2f::new(0., 2.),
            "actors",
            0,
            Color(255, 255, 255, 255),
            Vector2f::new(globals::TILE_SIZE, globals::TILE_SIZE)
        );
        self.spawn_interval = 8.;
    }
    fn update(&mut self, context: &mut Context_) {
        if context.input.is_key_down(rogalik_engine::input::VirtualKeyCode::W) {
            self.player.a.y = globals::LIFT_ACC;
        }

        if !self.player.grounded {
            if context.input.is_key_down(rogalik_engine::input::VirtualKeyCode::D) {
                self.player.a.x = globals::FLY_ACC;
            }
            if context.input.is_key_down(rogalik_engine::input::VirtualKeyCode::A) {
                self.player.a.x = -globals::FLY_ACC;
            }
        }

        if context.time.get_timer(self.animation_timer).unwrap().is_finished() {
            if self.player.a.y > 0. {
                self.player.sprite.frame = (self.player.sprite.frame + 1) % globals::ACTOR_FRAMES;
            }
            for passenger in self.passengers.iter_mut() {
                passenger.sprite.frame = (passenger.sprite.frame + 1) % globals::ACTOR_FRAMES;
                let offset = match passenger.animation_state {
                    passenger::PassengerAnimationState::Idle => 0,
                    passenger::PassengerAnimationState::Walking => 4,
                    passenger::PassengerAnimationState::Falling => 8,
                };
                passenger.sprite.frame += offset;
            }
        }
        if context.time.get_timer(self.spawn_timer).unwrap().is_finished() {
            passenger::try_spawn(self);
        }

        passenger::try_knock_down(self);
        passenger::try_load(self);
        passenger::try_unload(self);
        self.passengers.retain(|p| !passenger::should_remove(p));

        let obstacles = &self.board.colliders;
        player::move_player(&mut self.player, obstacles, context.time.get_delta());
        for passenger in self.passengers.iter_mut() {
            passenger::move_passenger(passenger, &self.player, context.time.get_delta());
        }
        render::render_sprites(self, context);
        ui::render_ui(self, context);
    }
}

fn main() {
    std::env::set_var("WINIT_UNIX_BACKEND", "x11");
    run();
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
fn run() {
    let state = State::default();
    let engine = Engine::new(state, 1024., 640., "Grota");
    engine.run();
}

fn load_assets(state: &mut State, context: &mut Context_) {
    let sprite_bytes_ascii = include_bytes!("../assets/ascii.png");
    let sprite_bytes_actors = include_bytes!("../assets/actors.png");
    let sprite_bytes_tiles = include_bytes!("../assets/tiles.png");

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
        sprite_bytes_tiles,
        4,
        4
    );
    state.textures.insert("ascii", t_0);
    state.textures.insert("actors", t_1);
    state.textures.insert("tiles", t_2);
    state.font = context.graphics.load_font(
        sprite_bytes_ascii,
        16,
        16
    );

    let camera_0 = context.graphics.create_camera(64.0, Vector2f::new(0., 4.));
    context.graphics.set_camera(camera_0);

    state.animation_timer = context.time.add_timer(globals::ANIMATION_TICK);
    state.spawn_timer = context.time.add_timer(globals::SPAWN_TICK);
    context.graphics.set_clear_color(Color(0, 0, 0, 255));
}