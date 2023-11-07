use rogalik_math::{
    aabb::Aabb,
    vectors::Vector2f
};
use rogalik_engine::Color;

use crate::globals::{
    GRAVITY_ACC, FLY_MAX_SPEED, LIFT_MAX_SPEED, HOR_DRAG, LIFT_ACC, DAMAGE_SPEED,
    HIT_IMMUNITY, TOLERANCE
};
use crate::passenger::Passenger;
use crate::sprite::DynamicSprite;
use crate::utils::almost_eq;

use super::State;

#[derive(Default)]
pub struct Stats {
    pub stamina: f32,
    pub stamina_use: f32,
    pub stamina_recovery: f32,
    pub score: u32,
    pub reputation: u32,
}
impl Stats {
    pub fn take_reputation(&mut self) {
        self.reputation = self.reputation.saturating_sub(1);
    }
}

#[derive(Default)]
pub struct Player {
    pub sprite: DynamicSprite,
    pub v: Vector2f,
    pub a: Vector2f,
    pub grounded: bool,
    pub immunity: f32,
    pub passenger: Option<Passenger>,
    pub stats: Stats
}
impl Player {
    pub fn new(
        position: Vector2f,
        z_index: i32,
        atlas: &'static str,
        sprite_index: usize,
        color: Color,
        collider_size: Vector2f
    ) -> Self {
        let sprite = DynamicSprite::new(
            position,
            z_index,
            atlas,
            sprite_index,
            color,
            collider_size,
            Vector2f::ZERO
        );
        Self {
            sprite,
            v: Vector2f::ZERO,
            a: Vector2f::ZERO,
            grounded: false,
            passenger: None,
            immunity: 0.,
            stats: Stats::default()
        }
    }
}

pub fn try_hit(player: &mut Player) -> bool {
    if player.immunity > TOLERANCE { return false; }
    player.immunity = HIT_IMMUNITY;
    player.stats.take_reputation();
    true
}

pub fn handle_lift(player: &mut Player, delta: f32, working: bool) {
    if !working {
        player.stats.stamina = 1.0_f32.min(player.stats.stamina + player.stats.stamina_recovery * delta);
        return
    }
    if player.stats.stamina > 0. {
        player.a.y = LIFT_ACC;
        player.stats.stamina = 0.0_f32.max(player.stats.stamina - player.stats.stamina_use * delta);
    }
}

pub fn update_player(state: &mut State, delta: f32) {
    state.player.immunity = 0.0_f32.max(
        state.player.immunity - delta
    );
    let blink = (state.player.immunity * 10.) as u32 % 2 == 1;
    state.player.sprite.color.3 = if blink { 0 } else { 255 };
    let obstacles = &state.board.colliders;
    state.player.v += delta * state.player.a;
    if move_y(&mut state.player, obstacles, delta) {
        if try_hit(&mut state.player) {
            state.audio.play("hit");
        }
    }
    move_x(&mut state.player, obstacles, delta);
}
fn move_y(player: &mut Player, obstacles: &Vec<Aabb>, delta: f32) -> bool {
    // returns true on damage
    // TODO - make a result struct or smth?
    player.grounded = false;
    player.v.y = player.v.y.min(LIFT_MAX_SPEED);
    player.a.y = -GRAVITY_ACC;

    let dy = delta * player.v.y;
    let colliders = collision(
        player.sprite.aabb_moved(Vector2f::new(0., dy)), obstacles
    );
    if colliders.len() == 0 {
        player.sprite.position.y += dy;
        return false;
    }
    let mut damage = false;
    // if collision on high speed, decr. rep
    if player.v.y.abs() > DAMAGE_SPEED {
        damage = true;
    }

    let y = if dy < TOLERANCE {
        player.grounded = true;
        colliders.iter()
            .map(|a| a.b.y).fold(f32::NEG_INFINITY, |a, b| a.max(b))
    } else {
        colliders.iter()
            .map(|a| a.a.y).fold(f32::INFINITY, |a, b| a.min(b))
        -player.sprite.collider_size.y    
    };
    player.sprite.position.y = y;
    player.v.y = 0.;
    damage
}
fn move_x(player: &mut Player, obstacles: &Vec<Aabb>, delta: f32) {
    player.a.x = match player.v.x {
        x if x < -TOLERANCE => HOR_DRAG,
        x if x > TOLERANCE => -HOR_DRAG,
        _ => 0.,
    };

    player.v.x = player.v.x.clamp(-FLY_MAX_SPEED, FLY_MAX_SPEED);
    if almost_eq(player.v.x, 0.) { player.v.x = 0. }
    let dx = delta * player.v.x;

    let colliders = collision(
        player.sprite.aabb_moved(Vector2f::new(dx, 0.)), obstacles
    );
    if colliders.len() == 0 {
        player.sprite.position.x += dx;
        return;
    }

    let x = if dx < 0. {
        colliders.iter()
            .map(|a| a.b.x).fold(f32::NEG_INFINITY, |a, b| a.max(b))
    } else {
        colliders.iter()
            .map(|a| a.a.x).fold(f32::INFINITY, |a, b| a.min(b))
        -player.sprite.collider_size.x
    };
    player.sprite.position.x = x;
}

fn collision(aabb: Aabb, obstacles: &Vec<Aabb>) -> Vec<Aabb> {
    obstacles.iter().filter(
            |o| aabb.intersects(o)
        )
        .map(|a| *a)
        .collect()
}