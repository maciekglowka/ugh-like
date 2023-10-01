use rogalik_engine::Color;
use rogalik_math::{
    aabb::Aabb,
    vectors::{Vector2f, Vector2i}
};

use crate::globals::{TOLERANCE, BIRD_SPEED, BIRD_MARGIN, BOARD_WIDTH, TILE_SIZE, HIT_IMMUNITY};
use crate::player::{Player, try_hit};
use crate::sprite::DynamicSprite;

use super::State;

pub enum CreatureKind {
    Bird
}

pub struct Creature {
    pub sprite: DynamicSprite,
    pub kind: CreatureKind,
    pub dir: Vector2f
}
impl Creature {
    pub fn new(
        kind: CreatureKind,
        position: Vector2f,
        atlas: &'static str,
        sprite_index: usize,
        color: Color,
        collider_size: Vector2f,
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
            kind,
            sprite,
            dir: Vector2f::RIGHT
        }
    }
}

pub fn update_creature(creature: &mut Creature, delta: f32) {
    match creature.kind {
        CreatureKind::Bird => fly_bird(creature, delta)
    }
}

fn fly_bird(creature: &mut Creature, delta: f32) {
    creature.sprite.position += creature.dir * BIRD_SPEED * delta;
    if creature.sprite.position.x > (BOARD_WIDTH + BIRD_MARGIN) as f32 / TILE_SIZE {
        creature.sprite.position.x = -(BIRD_MARGIN as f32) / TILE_SIZE;
    }
}

pub fn check_interactions(state: &mut State) {
    for creature in state.creatures.iter() {
        match creature.kind {
            CreatureKind::Bird => {
                if try_bird_collision(creature, &mut state.player) {
                    if try_hit(&mut state.player) {
                        state.audio.play("hit");
                    }
                }
            }
        }
    }
}

fn try_bird_collision(creature: &Creature, player: &mut Player) -> bool {
    if player.immunity > TOLERANCE { return false };
    if !player.sprite.aabb().intersects(&creature.sprite.aabb()) { return false }
    player.v.x += 2.0 * creature.dir.x * BIRD_SPEED;
    true
}