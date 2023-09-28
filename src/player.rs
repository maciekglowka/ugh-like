use rogalik_math::{
    aabb::Aabb,
    vectors::Vector2f
};
use rogalik_engine::{Color, ResourceId};

use crate::globals::{GRAVITY_ACC, FLY_MAX_SPEED, LIFT_MAX_SPEED, HOR_DRAG, TILE_SIZE};

#[derive(Default)]
pub struct Player {
    pub atlas: &'static str,
    pub sprite_index: usize,
    pub color: Color,
    pub frame: usize,
    pub position: Vector2f,
    pub size: Vector2f,
    pub v: Vector2f,
    pub a: Vector2f,
    pub grounded: bool
}
impl Player {
    pub fn new(
        position: Vector2f,
        atlas: &'static str,
        sprite_index: usize,
        color: Color
    ) -> Self {
        Self {
            atlas,
            sprite_index,
            position,
            color,
            frame: 0,
            v: Vector2f::ZERO,
            a: Vector2f::ZERO,
            size: Vector2f::new(TILE_SIZE, TILE_SIZE),
            grounded: false
        }
    }
    pub fn aabb(&self) -> Aabb {
        Aabb::new(self.position, self.position + self.size)
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
    let target = player.position + Vector2f::new(0., dy);
    let colliders = collision(Aabb::new(target, target + player.size), obstacles);
    if colliders.len() == 0 {
        player.position.y += dy;
        return;
    }

    let y = if dy < 0. {
        player.grounded = true;
        colliders.iter()
            .map(|a| a.b.y).fold(f32::NEG_INFINITY, |a, b| a.max(b))
    } else {
        colliders.iter()
            .map(|a| a.a.y).fold(f32::INFINITY, |a, b| a.min(b))
        -player.size.y    
    };
    player.position.y = y;
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

    let target = player.position + Vector2f::new(dx, 0.);
    let colliders = collision(Aabb::new(target, target + player.size), obstacles);
    if colliders.len() == 0 {
        player.position.x += dx;
        return;
    }

    let x = if dx < 0. {
        colliders.iter()
            .map(|a| a.b.x).fold(f32::NEG_INFINITY, |a, b| a.max(b))
    } else {
        colliders.iter()
            .map(|a| a.a.x).fold(f32::INFINITY, |a, b| a.min(b))
        -player.size.x
    };
    player.position.x = x;
}

fn collision(aabb: Aabb, obstacles: &Vec<Aabb>) -> Vec<Aabb> {
    obstacles.iter().filter(
            |o| aabb.intersects(o)
        )
        .map(|a| *a)
        .collect()
}