use rogalik_engine::Color;
use rogalik_math::{
    aabb::Aabb,
    vectors::Vector2f
};

pub struct Passenger {
    pub atlas: &'static str,
    pub sprite_index: usize,
    pub color: Color,
    pub frame: usize,
    pub position: Vector2f,
    pub size: Vector2f
}
impl Passenger {
    pub fn aabb(&self) -> Aabb {
        Aabb::new(self.position, self.position + self.size)
    }
}