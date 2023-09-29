use rogalik_math::{
    aabb::Aabb,
    vectors::Vector2f
};
use rogalik_engine::Color;

use crate::globals::{GRAVITY_ACC, FLY_MAX_SPEED, LIFT_MAX_SPEED, HOR_DRAG};
use crate::passenger::Passenger;
use crate::sprite::DynamicSprite;

#[derive(Default)]
pub struct Player {
    pub sprite: DynamicSprite,
    pub v: Vector2f,
    pub a: Vector2f,
    pub grounded: bool,
    pub passenger: Option<Passenger>
}
impl Player {
    pub fn new(
        position: Vector2f,
        atlas: &'static str,
        sprite_index: usize,
        color: Color,
        collider_size: Vector2f
    ) -> Self {
        let sprite = DynamicSprite::new(
            position,
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
            passenger: None
        }
    }
}

pub fn move_player(player: &mut Player, obstacles: &Vec<Aabb>, delta: f32) {
    player.v += delta * player.a;
    move_y(player, obstacles, delta);
    move_x(player, obstacles, delta);
    // println!("{:?}, {:?}", player.a, player.v);
}
fn move_y(player: &mut Player, obstacles: &Vec<Aabb>, delta: f32) {
    player.grounded = false;
    player.v.y = player.v.y.min(LIFT_MAX_SPEED);
    player.a.y = -GRAVITY_ACC;

    let dy = delta * player.v.y;
    let colliders = collision(
        player.sprite.aabb_moved(Vector2f::new(0., dy)), obstacles
    );
    if colliders.len() == 0 {
        player.sprite.position.y += dy;
        return;
    }

    let y = if dy < 0. {
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
}
fn move_x(player: &mut Player, obstacles: &Vec<Aabb>, delta: f32) {
    player.a.x = match player.v.x {
        x if x < 0. => HOR_DRAG,
        x if x > 0. => -HOR_DRAG,
        _ => 0.,
    };

    player.v.x = player.v.x.clamp(-FLY_MAX_SPEED, FLY_MAX_SPEED);
    let dx = delta * player.v.x;

    // let target = player.sprite.position + Vector2f::new(dx, 0.);
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