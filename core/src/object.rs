use super::*;

pub trait Object {
    fn collision(&self, light: &Light) -> (Option<Point>, Option<f32>);
    fn color(&self) -> Color;
    fn normal(&self, point: Point) -> Vector;
    fn ior(&self) -> f32;
}
