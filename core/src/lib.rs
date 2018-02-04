extern crate rand;
extern crate image;
extern crate rayon;

mod color;
mod light;
mod object;
mod pixel;
mod point;
mod scene;
mod vector;

pub use color::*;
pub use light::*;
pub use object::*;
pub use pixel::*;
pub use point::*;
pub use scene::*;
pub use vector::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    #[test]
    fn test() {
        let scene = Scene::default();
        match scene.render().save("./1.jpg") {
            Err(e) => println!("save error: {}", e.description()),
            _ => {}
        }
    }
}
