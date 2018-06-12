extern crate core;

use std::error::Error;

use core::{Color, Light, Object, Point, Scene, Vector};

#[derive(Debug)]
struct Circle {
    pub center: Point,
    pub radius: f32,
    pub color: Color,
}

impl Object for Circle {
    fn collision(&self, light: &Light) -> (Option<Point>, Option<f32>) {
        let ux = self.center.x - light.start.x;
        let uy = self.center.y - light.start.y;
        let distance = (ux * ux + uy * uy).sqrt();
        if distance < self.radius {
            return (Some(Point { x: 0.0, y: 0.0 }), Some(-1.0));
        }

        let vlr = Vector {
            x: self.center.x - light.start.x,
            y: self.center.y - light.start.y,
        };
        let lrl = light.direction.clone() * vlr.clone();
        let drl = (vlr.magnitude() - lrl * lrl).sqrt();
        if drl >= self.radius {
            (None, None)
        } else {
            let dlc = drl - (self.radius * self.radius - drl * drl).sqrt();
            (
                Some(light.start.clone() + light.direction.clone() * dlc),
                Some(dlc),
            )
        }
    }
    fn color(&self) -> Color {
        self.color.clone()
    }
}

fn main() {
    let mut scene = Scene::default();
    scene.object.push(Box::new(Circle {
        center: Point { x: 0.2, y: 0.2 },
        radius: 0.1,
        color: Color::new(255, 0, 0),
    }));
    scene.object.push(Box::new(Circle {
        center: Point { x: 0.8, y: 0.3 },
        radius: 0.1,
        color: Color::new(0, 255, 0),
    }));
    scene.object.push(Box::new(Circle {
        center: Point { x: 0.6, y: 0.5 },
        radius: 0.1,
        color: Color::new(0, 0, 255),
    }));
    match scene.render().save("./1.png") {
        Err(e) => println!("save error: {}", e.description()),
        _ => {}
    }
}
