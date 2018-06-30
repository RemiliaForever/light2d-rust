use super::*;

#[derive(Debug, Clone)]
pub struct Light {
    pub start: Point,
    pub direction: Vector,
}

impl Light {
    pub fn from_theta(start: &Point, theta: f32) -> Light {
        Light {
            start: start.clone(),
            direction: Vector::from_theta(theta),
        }
    }
}
