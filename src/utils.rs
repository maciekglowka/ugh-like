use crate::globals::TOLERANCE;

pub fn almost_eq(a: f32, b: f32) -> bool {
    (a - b).abs() <= TOLERANCE
}