use rogalik_math::vectors::Vector2f;
use crate::globals::TOLERANCE;

pub fn almost_eq(a: f32, b: f32) -> bool {
    (a - b).abs() <= TOLERANCE
}

pub fn to_roman(a: u32) -> &'static str {
    // TODO expand if needed for more gates
    match a {
        1 => "I",
        2 => "II",
        3 => "III",
        4 => "IV",
        5 => "V",
        6 => "VI",
        7 => "VII",
        8 => "VIII",
        9 => "IX",
        10 => "X",
        _ => ""
    }
}

pub fn pixel_perfect(v: Vector2f) -> Vector2f {
    let scale = crate::globals::PIXEL_SCALE;
    Vector2f::new((v.x * scale).round() / scale, (v.y * scale).round() / scale)
}