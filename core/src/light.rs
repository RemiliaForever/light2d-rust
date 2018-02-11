use super::*;

#[derive(Debug)]
pub struct Light {
    pub start: Point,
    pub direction: Vector,
}

impl Light {
    pub fn trace(&mut self, distance: f32) {
        self.start.x += distance * self.direction.x;
        self.start.y += distance * self.direction.y;
    }
}
