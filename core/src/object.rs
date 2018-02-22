use super::*;

pub trait Object {
    fn sdf(&self, &Light) -> f32;
    fn color(&self) -> Color;
}
