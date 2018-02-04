use super::*;

#[derive(Debug)]
pub struct Light {
    pub start: Point,
    pub direction: Vector,
}

impl Light {
    pub fn trace() -> Color {
        Color {
            red: 255,
            green: 255,
            blue: 255,
        }
    }
}
