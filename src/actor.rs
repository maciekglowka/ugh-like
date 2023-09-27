use rogalik_math::{
    aabb::Aabb,
    vectors::Vector2f
};
use rogalik_engine::{Color, ResourceId};

use crate::globals::{TILE_SIZE, GRAVITY};

#[derive(Default)]
pub struct Actor {
    pub atlas: &'static str,
    pub sprite_index: usize,
    pub color: Color,
    pub frame: usize,
    pub position: Vector2f,
    pub size: Vector2f,
    pub v: Vector2f,
}
impl Actor {
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
            size: Vector2f::new(TILE_SIZE, TILE_SIZE)
        }
    }
    pub fn aabb(&self) -> Aabb {
        Aabb::new(self.position, self.position + self.size)
    }
}

pub fn move_actor(actor: &mut Actor, obstacles: &Vec<Aabb>, delta: f32) {
    // let dy = - delta * GRAVITY;
    // let target_y = actor.position + Vector2f::new(0., dy);
    // if !collision(
    //     Aabb::new(target_y, target_y + actor.size),
    //     obstacles
    // ) {
    //     actor.position = target_y;
    // }
    move_y(actor, obstacles, delta);
    move_x(actor, obstacles, delta);
}
pub fn move_y(actor: &mut Actor, obstacles: &Vec<Aabb>, delta: f32) {
    let dy = delta * (actor.v.y - GRAVITY);
    let target = actor.position + Vector2f::new(0., dy);
    let colliders = collision(Aabb::new(target, target + actor.size), obstacles);
    if colliders.len() == 0 {
        actor.position.y += dy;
        return;
    }

    let y = if dy < 0. {
        colliders.iter()
            .map(|a| a.b.y).fold(f32::NEG_INFINITY, |a, b| a.max(b))
    } else {
        colliders.iter()
            .map(|a| a.a.y).fold(f32::INFINITY, |a, b| a.min(b))
        -actor.size.y    
    };
    actor.position.y = y;
}
pub fn move_x(actor: &mut Actor, obstacles: &Vec<Aabb>, delta: f32) {
    let dx = delta * actor.v.x;
    let target = actor.position + Vector2f::new(dx, 0.);
    let colliders = collision(Aabb::new(target, target + actor.size), obstacles);
    if colliders.len() == 0 {
        actor.position.x += dx;
        return;
    }

    let x = if dx < 0. {
        colliders.iter()
            .map(|a| a.b.x).fold(f32::NEG_INFINITY, |a, b| a.max(b))
    } else {
        colliders.iter()
            .map(|a| a.a.x).fold(f32::INFINITY, |a, b| a.min(b))
        -actor.size.x
    };
    actor.position.x = x;
}

fn collision(aabb: Aabb, obstacles: &Vec<Aabb>) -> Vec<Aabb> {
    obstacles.iter().filter(
            |o| aabb.intersects(o)
        )
        .map(|a| *a)
        .collect()
}