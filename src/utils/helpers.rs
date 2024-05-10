pub fn lerpf(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

pub fn clampf(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
