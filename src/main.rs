extern crate core;

use std::error::Error;

use core::{Color, Light, Object, Point, Scene};

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

fn main() {
    let mut scene = Scene::default();
    scene.object.push(Box::new(Circle {
        center: Point { x: 0.4, y: 0.5 },
        radius: 0.1,
        color: Color::new(511, 0, 0),
    }));
    scene.object.push(Box::new(Circle {
        center: Point { x: 0.6, y: 0.5 },
        radius: 0.1,
        color: Color::new(0, 1023, 0),
    }));
    scene.object.push(Box::new(Circle {
        center: Point { x: 0.5, y: 0.5 },
        radius: 0.1,
        color: Color::new(0, 0, 511),
    }));
    match scene.render().save("./1.png") {
        Err(e) => println!("save error: {}", e.description()),
        _ => {}
    }
}
