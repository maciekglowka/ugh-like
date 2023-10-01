use rogalik_engine::{Context, GraphicsContext, Engine, Game, ResourceId, Color};
use rogalik_math::vectors::Vector2f;
use rogalik_wgpu::WgpuContext;
use std::collections::{HashMap, VecDeque};

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

type Context_ = Context<WgpuContext>;

mod audio;
mod board;
mod creatures;
mod globals;
mod passenger;
mod player;
mod render;
mod sprite;
mod ui;
mod utils;

#[derive(Default)]
enum GameState {
    #[default]
    Init,
    Play,
    GameOver
}

#[derive(Default)]
pub struct State {
    audio: audio::AudioContext,
    game_state: GameState,
    level_data: HashMap<&'static str, &'static str>,
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
    }
    fn update(&mut self, context: &mut Context_) {
        match self.game_state {
            GameState::Init => {
                game_init(self, context);
            }
            GameState::Play => {
                game_loop(self, context);
                render::render_sprites(self, context);
                ui::render_game_ui(self, context);
            },
            GameState::GameOver => {
                game_over_loop(self, context);
                render::render_sprites(self, context);
                ui::render_game_ui(self, context);
                ui::render_game_over(self, context);
            }
        }
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

fn game_loop(state: &mut State, context: &mut Context_) {
    // check loose condition
    if state.player.stats.reputation == 0 {
        state.game_state = GameState::GameOver;
        return
    }
    update_difficulty(state);

    if context.input.is_key_down(rogalik_engine::input::VirtualKeyCode::W) {
        player::handle_lift(&mut state.player, context.time.get_delta(), true);

    } else {
        player::handle_lift(&mut state.player, context.time.get_delta(), false);
    }

    if !state.player.grounded {
        if context.input.is_key_down(rogalik_engine::input::VirtualKeyCode::D) {
            state.player.a.x = globals::FLY_ACC;
        }
        if context.input.is_key_down(rogalik_engine::input::VirtualKeyCode::A) {
            state.player.a.x = -globals::FLY_ACC;
        }
    }

    if context.time.get_timer(state.animation_timer).unwrap().is_finished() {
        if state.player.a.y > 0. {
            state.player.sprite.frame = (state.player.sprite.frame + 1) % globals::ACTOR_FRAMES;
        }
        for passenger in state.passengers.iter_mut() {
            passenger.sprite.frame = (passenger.sprite.frame + 1) % globals::ACTOR_FRAMES;
            let offset = match passenger.animation_state {
                passenger::PassengerAnimationState::Idle => 0,
                passenger::PassengerAnimationState::Walking => 4,
                passenger::PassengerAnimationState::Falling => 8,
            };
            passenger.sprite.frame += offset;
        }
    }
    if context.time.get_timer(state.spawn_timer).unwrap().is_finished() {
        passenger::try_spawn(state);
    }

    passenger::handle_waiting(state, context.time.get_delta());
    passenger::try_knock_down(state);
    passenger::try_load(state);
    passenger::try_unload(state);
    state.passengers.retain(|p| !passenger::should_remove(p));

    player::move_player(state, context.time.get_delta());
    for passenger in state.passengers.iter_mut() {
        passenger::move_passenger(passenger, &state.player, context.time.get_delta());
    }
}

fn update_difficulty(state: &mut State) {
    let decr = state.player.stats.score / globals::SPAWN_DROP_EVERY;
    state.spawn_interval = 1.0_f32.max(globals::BASE_SPAWN_INTERVAL - decr as f32);
}

fn game_init(state: &mut State, context: &mut Context_) {
    reinit(state, context);
    load_level(state, context, "playground");
    state.game_state = GameState::Play;
}

fn game_over_loop(state: &mut State, context: &mut Context_) {
    if context.input.is_key_down(rogalik_engine::input::VirtualKeyCode::Space) {
        state.game_state = GameState::Init;
    };
}

fn load_assets(state: &mut State, context: &mut Context_) {
    let playground_lvl = include_str!("../assets/playground.lvl");
    state.level_data.insert("playground", playground_lvl);

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

    state.audio = audio::get_audio_context();

    let camera_0 = context.graphics.create_camera(
        globals::PIXEL_SCALE, Vector2f::new(globals::BOARD_WIDTH as f32 / 2., globals::BOARD_HEIGHT as f32 / 2.)
    );
    context.graphics.set_camera(camera_0);

    state.animation_timer = context.time.add_timer(globals::ANIMATION_TICK);
    state.spawn_timer = context.time.add_timer(globals::SPAWN_TICK);
    context.graphics.set_clear_color(Color(3, 2, 2, 255));
}

fn load_level(state: &mut State, context: &mut Context_, name: &str) {
    let data = state.level_data.get(name).expect("Level data not found!");
    state.board = board::generate_board(data);
}

fn reinit(state: &mut State, context: &mut Context_) {
    // reinitialize the game state for a fresh game or restart
    state.player = player::Player::new(
        Vector2f::new((globals::BOARD_WIDTH / 2) as f32, 2.),
        "actors",
        0,
        Color(255, 255, 255, 255),
        Vector2f::new(globals::TILE_SIZE, globals::TILE_SIZE)
    );
    state.player.stats.reputation = globals::BASE_REPUTATION;
    state.player.stats.stamina_use = globals::BASE_STAMINA_USE;
    state.player.stats.stamina_recovery = globals::BASE_STAMINA_RECOVERY;
    state.player.stats.stamina = 1.0;
    state.player.passenger = None;
    state.passengers = Vec::new();
    state.spawn_interval = globals::BASE_SPAWN_INTERVAL;
    state.since_spawn = 0.;
}