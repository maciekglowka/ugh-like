pub const TILE_SIZE: f32 = 1.;
pub const PIXEL_SCALE: f32 = 64.;
pub const PASSENGER_HEIGHT: f32 = 0.75;
pub const PASSENGER_WIDTH: f32 = 0.5;
pub const BOARD_WIDTH: u32 = 16;
pub const BOARD_HEIGHT: u32 = 10;

pub const BACKGROUND_Z: i32 = -10;
pub const TILE_Z: i32 = 0;
pub const PASSENGER_Z: i32 = 10;
pub const PLAYER_Z: i32 = 15;
pub const CREATURE_Z: i32 = 17;
pub const UI_BG_Z: i32 = 20;
pub const UI_Z: i32 = 25;

pub const BASE_REPUTATION: u32 = 5;
pub const BASE_STAMINA_USE: f32 = 0.1;
pub const BASE_STAMINA_RECOVERY: f32 = 0.04;

pub const BASE_SPAWN_INTERVAL: f32 = 8.;
pub const SPAWN_DROP_EVERY: u32 = 10;

pub const TOLERANCE: f32 = 0.01;
pub const PASSENGER_LOAD_DIST: f32 = 3.;
pub const PASSENGER_WALK_SPEED: f32 = 2.;
pub const PASSENGER_FALL_SPEED: f32 = 5.;
pub const PASSENGER_KNOCK_DOWN_SPEED: f32 = 2.5;
pub const PASSENGER_MAX_WAIT: f32 = 12.5;

pub const FLY_ACC: f32 = 6.;
pub const FLY_MAX_SPEED: f32 = 4.;
pub const HOR_DRAG: f32 = 4.;
pub const LIFT_ACC: f32 = 3.;
pub const LIFT_MAX_SPEED: f32 = 4.;
pub const GRAVITY_ACC: f32 = 5.;
pub const DAMAGE_SPEED: f32 = 5.;

pub const BIRD_SPEED: f32 = 5.;
pub const BIRD_MARGIN: u32 = 4;
pub const HIT_IMMUNITY: f32 = 2.;
pub const MAMMOTH_BLOW_SPEED: f32 = 2.;
pub const MAMMOTH_BLOW_V_OFFSET: f32 = 1.;
pub const MAMMOTH_BLOW_V_SIZE: f32 = 0.5;

pub const ACTOR_FRAMES: usize = 4;
pub const ANIMATION_TICK: f32 = 0.1;
pub const SPAWN_TICK: f32 = 1.;