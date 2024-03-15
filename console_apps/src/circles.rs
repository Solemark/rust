use std::f32::consts::PI;

#[allow(dead_code)]
pub fn area(radius: f32) -> f32 {
    if radius < 0.0 {
        return 0.0;
    }
    PI * (radius * radius)
}

#[allow(dead_code)]
pub fn perimeter(radius: f32) -> f32 {
    if radius < 0.0 {
        return 0.0;
    }
    2.0 * PI * radius
}
