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

    pub fn normalize(&mut self) {
        let scale = self.x * self.x + self.y * self.y;
        self.x = self.x / scale;
        self.y = self.y / scale;
    }
}
