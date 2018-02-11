use super::*;

pub trait Object {
    fn sdf(&self, &Light) -> f32;
    fn color(&self) -> Color;
}

#[derive(Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: f32,
    pub color: Color,
}
impl Object for Circle {
    fn sdf(&self, light: &Light) -> f32 {
        let ux = self.center.x - light.start.x;
        let uy = self.center.y - light.start.y;
        (ux * ux + uy * uy).sqrt() - self.radius
    }
    fn color(&self) -> Color {
        self.color.clone()
    }
}
