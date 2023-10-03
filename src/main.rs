use rogalik_engine::{Context, GraphicsContext, EngineBuilder, Game, ResourceId, Color};
use rogalik_engine::input::VirtualKeyCode;
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
    MainMenu,
    Init,
    Play,
    GameOver
}

#[derive(Default)]
pub struct State {
    audio: audio::AudioContext,
    camera_main: ResourceId,
    game_state: GameState,
    level: &'static str,
    level_data: HashMap<&'static str, &'static str>,
    board: board::Board,
    animation_timer: ResourceId,
    textures: HashMap<&'static str, ResourceId>,
    font: ResourceId,
    player: player::Player,
    passengers: Vec<passenger::Passenger>,
    creatures: Vec<creatures::Creature>,
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
            GameState::MainMenu => {
                ui::render_main_menu(self, context);
            },
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
    let engine = EngineBuilder::new()
        .with_title("Grrr!".to_string())
        .with_logical_size(1024., 640.)
        .build(state);
    
    //state, 1024, 640, "Grota");
    engine.run();
}

fn game_loop(state: &mut State, context: &mut Context_) {
    // check loose condition
    if state.player.stats.reputation == 0 {
        state.game_state = GameState::GameOver;
        return
    }
    update_difficulty(state);

    if context.input.is_key_down(VirtualKeyCode::W) || context.input.is_key_down(VirtualKeyCode::Up) {
        player::handle_lift(&mut state.player, context.time.get_delta(), true);

    } else {
        player::handle_lift(&mut state.player, context.time.get_delta(), false);
    }

    if !state.player.grounded {
        if context.input.is_key_down(VirtualKeyCode::D) || context.input.is_key_down(VirtualKeyCode::Right) {
            state.player.a.x = globals::FLY_ACC;
        }
        if context.input.is_key_down(VirtualKeyCode::A) || context.input.is_key_down(VirtualKeyCode::Left) {
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
        for creature in state.creatures.iter_mut() {
            creature.sprite.frame = (creature.sprite.frame + 1) % globals::ACTOR_FRAMES;
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

    creatures::check_interactions(state);

    player::update_player(state, context.time.get_delta());
    for passenger in state.passengers.iter_mut() {
        passenger::move_passenger(passenger, &state.player, context.time.get_delta());
    }
    for creature in state.creatures.iter_mut() {
        creatures::update_creature(creature, context.time.get_delta());
    }
}

fn update_difficulty(state: &mut State) {
    let decr = state.player.stats.score / globals::SPAWN_DROP_EVERY;
    state.spawn_interval = 1.0_f32.max(globals::BASE_SPAWN_INTERVAL - decr as f32);
}

fn game_init(state: &mut State, context: &mut Context_) {
    reinit(state, context);
    load_level(state, context, state.level);
    state.game_state = GameState::Play;
}

fn game_over_loop(state: &mut State, context: &mut Context_) {
    if context.input.is_key_down(rogalik_engine::input::VirtualKeyCode::Space) {
        state.game_state = GameState::MainMenu;
    };
}

fn load_assets(state: &mut State, context: &mut Context_) {
    state.level_data.insert("Tricity", include_str!("../assets/tricity.lvl"));
    state.level_data.insert("Birdy", include_str!("../assets/birdy.lvl"));
    state.level_data.insert("Mammoth Hotel", include_str!("../assets/mammoths.lvl"));

    state.textures.insert(
        "ascii",
        context.graphics.load_sprite_atlas(
            include_bytes!("../assets/ascii.png"), 16, 16
        )
    );
    state.textures.insert(
        "tiles",
        context.graphics.load_sprite_atlas(
            include_bytes!("../assets/tiles.png"), 4, 4
        )
    );
    state.textures.insert(
        "actors",
        context.graphics.load_sprite_atlas(
            include_bytes!("../assets/actors.png"), 4, 4
        )
    );
    state.textures.insert(
        "creatures",
        context.graphics.load_sprite_atlas(
            include_bytes!("../assets/creatures.png"), 4, 4
        )
    );

    state.font = context.graphics.load_font(
        include_bytes!("../assets/ascii.png"),
        16,
        16
    );

    state.audio = audio::get_audio_context();

    state.camera_main = context.graphics.create_camera(
        globals::PIXEL_SCALE, Vector2f::new(globals::BOARD_WIDTH as f32 / 2., globals::BOARD_HEIGHT as f32 / 2.)
    );
    context.graphics.set_camera(state.camera_main);

    state.animation_timer = context.time.add_timer(globals::ANIMATION_TICK);
    state.spawn_timer = context.time.add_timer(globals::SPAWN_TICK);
    context.graphics.set_clear_color(Color(3, 2, 2, 255));
}

fn load_level(state: &mut State, context: &mut Context_, name: &str) {
    let data = state.level_data.get(name).expect("Level data not found!");
    (state.board, state.creatures) = board::generate_board(data);
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