extern crate core;

use core::{Color, Light, Object, Point, Vector};

#[derive(Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: f32,
    pub color: Color,
    pub ior: f32,
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
        if lrl <= 0.0 {
            (None, None)
        } else {
            let drl = (vlr.magnitude() - lrl * lrl).sqrt();
            if drl >= self.radius {
                (None, None)
            } else {
                let dlc = lrl - (self.radius * self.radius - drl * drl).sqrt();
                (
                    Some(light.start.clone() + light.direction.clone() * dlc),
                    Some(dlc),
                )
            }
        }
    }

    fn color(&self) -> Color {
        self.color.clone()
    }

    fn normal(&self, point: Point) -> Vector {
        point - self.center.clone()
    }

    fn ior(&self) -> f32 {
        self.ior
    }
}
