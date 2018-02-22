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

    #[derive(Debug)]
    struct Circle {
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
    #[test]
    fn test() {
        let mut scene = Scene::default();
        scene.object.push(Box::new(Circle {
            center: Point { x: 0.5, y: 0.5 },
            radius: 0.05,
            color: Color::new(511, 511, 511),
        }));
        scene.object.push(Box::new(Circle {
            center: Point { x: 0.8, y: 0.8 },
            radius: 0.02,
            color: Color::new(255, 255, 255),
        }));
        match scene.render().save("./1.jpg") {
            Err(e) => println!("save error: {}", e.description()),
            _ => {}
        }
    }
}
