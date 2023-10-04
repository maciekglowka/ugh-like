use rogalik_engine::{Color, GraphicsContext, Params2d, ResourceId};
use rogalik_math::vectors::Vector2f;

use super::{Context_, State};
use crate::globals::{TILE_SIZE, PIXEL_SCALE, BASE_REPUTATION, PASSENGER_MAX_WAIT};
use crate::passenger::PassengerState;
use crate::utils::to_roman;

const UI_BG: Color = Color(85, 113, 119, 255);
const UI_RED: Color = Color(152, 77, 77, 255);

pub fn render_game_ui(state: &State, context: &mut Context_) {
    render_passenger_targets(state, context);
    render_status_bar(state, context);
}

fn render_passenger_targets(state: &State, context: &mut Context_) {
    for passenger in state.passengers.iter() {
        if let PassengerState::Waiting(time) = passenger.state {
            let color = if time > 0.5 * PASSENGER_MAX_WAIT {
                UI_RED
            } else {
                UI_BG
            };
            context.graphics.draw_atlas_sprite(
                state.textures["ascii"],
                219,
                passenger.sprite.centre() + Vector2f::new(- 0.4 * TILE_SIZE, 0.8 * TILE_SIZE),
                Vector2f::new(0.8 * TILE_SIZE, 0.5 * TILE_SIZE),
                Params2d { color, ..Default::default() }
            );
            let t = to_roman(passenger.target_gate + 1);
            let dx = 0.5 * t.len() as f32 * 0.25 * TILE_SIZE;
            context.graphics.draw_text(
                state.font,
                t,
                passenger.sprite.centre() + Vector2f::new(-dx, 0.9 * TILE_SIZE),
                0.25 * TILE_SIZE,
                Params2d { color: Color(0, 0, 0, 255), ..Default::default() }
            );
        }
    }
}

fn render_status_bar(state: &State, context: &mut Context_) {
    let top = context.get_logical_size().y / PIXEL_SCALE;
    let base = Vector2f::new(0.25, top - 0.5 * TILE_SIZE);
    let stamina_width = 4. * TILE_SIZE;
    let height = 0.25 * TILE_SIZE;
    let margin = 0.25 * TILE_SIZE;
    let mut offset = 0.;

    // draw reputation

    for i in 0..state.player.stats.reputation {
        context.graphics.draw_atlas_sprite(
            state.textures["ascii"],
            3,
            base + Vector2f::new(offset + i as f32 * height, 0.),
            Vector2f::new(height, height),
            Params2d { color: UI_RED, ..Default::default() }
        );
    }
    offset += BASE_REPUTATION as f32 * height + margin;

    // draw stamina bar
    context.graphics.draw_atlas_sprite(
        state.textures["ascii"],
        219,
        base + Vector2f::new(offset, 0.),
        Vector2f::new(stamina_width, height),
        Params2d { color: UI_BG, ..Default::default() }
    );
    context.graphics.draw_atlas_sprite(
        state.textures["ascii"],
        219,
        base + Vector2f::new(offset, 0.),
        Vector2f::new(state.player.stats.stamina * stamina_width, height),
        Params2d { color: UI_RED, ..Default::default() }
    );
    offset += stamina_width + margin;

    // draw score
    let score_text = format!("Score: {}", state.player.stats.score);
    context.graphics.draw_text(
        state.font,
        &score_text,
        base + Vector2f::new(offset, 0.),
        height,
        Params2d { color: UI_BG, ..Default::default() }
    );
    offset += margin + context.graphics.text_dimensions(state.font, &score_text, height).x;

    // draw load status
    if let Some(passenger) = &state.player.passenger {
        context.graphics.draw_atlas_sprite(
            state.textures["ascii"],
            219,
            base + Vector2f::new(offset, -0.5 * height),
            Vector2f::new(3. * height, 2. * height),
            Params2d { color: UI_BG, ..Default::default() }
        );
        render_centered_text(
            base + Vector2f::new(offset + 1.5 * height, 0.),
            &format!("{}", to_roman(passenger.target_gate + 1)),
            height,
            Color(0, 0, 0, 255),
            state,
            context
        );
    }
}

pub fn render_game_over(state: &State, context: &mut Context_) {
    let vs = context.get_logical_size() / PIXEL_SCALE;
    let centre = Vector2f::new(
        0.5 * vs.x,
        0.5 * vs.y,
    );
    render_centered_text(centre, "GAME OVER", TILE_SIZE, UI_BG, state, context);
    render_centered_text(
        centre - Vector2f::new(0., TILE_SIZE * 1.25),
        &format!("Passengers delivered: {}", state.player.stats.score),
        0.5 *TILE_SIZE,
        UI_BG,
        state,
        context
    );
    render_centered_text(
        centre - Vector2f::new(0., TILE_SIZE * 2.),
        "(press spacebar)",
        0.5 *TILE_SIZE,
        UI_BG,
        state,
        context
    );
}

fn render_centered_text(
    v: Vector2f,
    t: &str,
    height: f32,
    color: Color,
    state: &State,
    context: &mut Context_
) {
    let dim = context.graphics.text_dimensions(state.font, &t, height);
    context.graphics.draw_text(
        state.font,
        &t,
        v - Vector2f::new(0.5 * dim.x, 0.),
        height,
        Params2d { color, ..Default::default() }
    );
}

pub fn render_main_menu(state: &mut State, context: &mut Context_) {
    let button_height = TILE_SIZE;
    let button_width = TILE_SIZE * 8.;
    let vs = context.get_logical_size();
    let top = Vector2f::new(
        0.5 * vs.x / PIXEL_SCALE,
        vs.y / PIXEL_SCALE,
    );

    render_centered_text(
        top - Vector2f::new(0., 2. * TILE_SIZE),
        "Grrr!",
        TILE_SIZE,
        UI_BG,
        state,
        context
    );

    let base = Vector2f::new(
        top.x - 0.5 * button_width,
        top.y - 3.5 * TILE_SIZE
    );
    for (i, level) in state.level_data.keys().enumerate() {
        let button = Button::new(
                base.x,
                base.y - i as f32 * 1.25 * button_height,
                button_width,
                button_height
            )
            .with_text(level.to_string())
            .with_color(Color(255, 255, 255, 255));
        button.draw(state, context);
        if button.clicked(state.camera_main, context) {
            state.level = level;
            state.game_state = super::GameState::Init;
        }
    }
}

#[derive(Default)]
pub struct Button {
    origin: Vector2f,
    w: f32,
    h: f32,
    text: String,
    color: Color
}
impl Button {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Button { 
            origin: Vector2f::new(x, y),
            w,
            h,
            ..Default::default()
        }
    }
    pub fn with_text(mut self, text: String) -> Self {
        self.text = text;
        self
    }
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    pub fn draw(&self, state: &State, context: &mut Context_) {
        context.graphics.draw_atlas_sprite(
            state.textures["ascii"],
            219,
            self.origin,
            Vector2f::new(self.w, self.h),
            Params2d { color: UI_BG, ..Default::default() }
        );
        render_centered_text(
            self.origin + Vector2f::new(self.w / 2., 0.25 * self.h),
            &self.text,
            self.h / 2.,
            self.color,
            state,
            context
        );
    }
    pub fn clicked(&self, camera_id: ResourceId, context: &Context_) -> bool {
        if !context.input.is_mouse_button_down(rogalik_engine::input::MouseButton::Left) { 
            return false;
        }
        if let Some(camera) = context.graphics.get_camera(camera_id) {
            let m = context.input.get_mouse_physical_position();
            let v = camera.camera_to_world(m);
            return v.x >= self.origin.x && v.y >= self.origin.y &&
                v.x <= self.origin.x + self.w && v.y <= self.origin.y + self.h;
        }
        false
    }
}