const PI: f32 = std::f32::consts::PI;

#[allow(dead_code)]
pub fn area(radius: f32) -> f32 {
    if validate(radius) {
        PI * radius.powi(2)
    } else {
        0.0
    }
}

#[allow(dead_code)]
pub fn perimeter(radius: f32) -> f32 {
    if validate(radius) {
        2.0 * PI * radius
    } else {
        0.0
    }
}

fn validate(radius: f32) -> bool {
    radius >= 0.0
}
