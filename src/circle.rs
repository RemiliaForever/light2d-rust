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
        let vlr: Vector = &self.center - &light.start;
        if vlr.magnitude().sqrt() < self.radius {
            return (Some(Point { x: 0.0, y: 0.0 }), Some(-1.0));
        }
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

    fn normal(&self, point: &Point) -> Vector {
        (point - &self.center).normalvec()
    }

    fn ior(&self) -> f32 {
        self.ior
    }
}
