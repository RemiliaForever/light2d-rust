use super::*;

pub trait Object {
    fn sdf(light: Light) -> f32;
    fn color() -> Color;
}
