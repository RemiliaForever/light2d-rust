use super::*;
use std::ops::{Add, Sub};

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, v: Vector) -> Point {
        Point {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, s: Point) -> Vector {
        Vector {
            x:self.x - s.x,
            y:self.y - s.y
        }
    }
}
