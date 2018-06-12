use super::*;

pub trait Object {
    fn collision(&self, light: &Light) -> (Option<Point>, Option<f32>);
    fn color(&self) -> Color;
}
