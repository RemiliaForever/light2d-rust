use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Vector {
        let mut v = Vector { x, y };
        v.normalize();
        v
    }
    pub fn from_theta(theta: f32) -> Vector {
        Vector {
            x: theta.cos(),
            y: theta.sin(),
        }
    }

    pub fn magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn normalvec(&self) -> Vector {
        let mut vec = self.clone();
        vec.normalize();
        vec
    }

    pub fn normalize(&mut self) {
        let scale = self.x * self.x + self.y * self.y;
        self.x = self.x / scale;
        self.y = self.y / scale;
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul for Vector {
    type Output = f32;

    fn mul(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl Div<f32> for Vector {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, vec: Vector) -> Vector {
        Vector {
            x: vec.x * self,
            y: vec.y * self,
        }
    }
}
