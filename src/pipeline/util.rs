use nalgebra::Vector2;

pub fn vector2f_lerp(v1: &Vector2<f64>, v2: &Vector2<f64>, p: f64) -> Vector2<f64> {
    v1 + (v2 - v1) * p
}
pub fn vector2i_lerp(v1: &Vector2<i32>, v2: &Vector2<i32>, p: f64) -> Vector2<i32> {
    let res = vector2f_lerp(&Vector2::new(v1.x as f64, v1.y as f64), &Vector2::new(v2.x as f64, v2.y as f64), p);
    Vector2::new(res.x.round() as i32, res.y.round() as i32)
}