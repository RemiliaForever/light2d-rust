use std::default::Default;
use std::error::Error;
use std::f32::consts::PI;
use std::time::SystemTime;

use image::{ImageBuffer, RgbImage};
use rand::{rngs::SmallRng, FromEntropy, Rng};
use rayon::prelude::*;

use super::*;

pub struct Scene {
    pub config: SceneConfig,
    pub size: u32,
    pub object: Vec<Box<Object + Sync>>,
}

impl Default for Scene {
    fn default() -> Scene {
        Scene {
            config: SceneConfig::default(),
            size: 1024,
            object: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct SceneConfig {
    pub max_step: u32,
    pub n_sampling: u32,
}

impl Default for SceneConfig {
    fn default() -> SceneConfig {
        SceneConfig {
            max_step: 10,
            n_sampling: 512,
        }
    }
}

impl Scene {
    fn trace(&self, light: Light, weight: f32, depth: u32) -> (f32, f32, f32) {
        if depth >= 10 || weight <= 0.0 {
            return (0.0, 0.0, 0.0);
        }
        let mut result: (Option<&Box<Object + Sync>>, Option<Point>, Option<f32>) =
            (None, None, None);
        (&self.object).into_iter().for_each(|x| {
            match x.collision(&light) {
                (Some(p), Some(d)) => {
                    if let Some(od) = result.2 {
                        if d <= od {
                            result = (Some(x), Some(p), Some(d));
                        }
                    } else {
                        result = (Some(x), Some(p), Some(d));
                    }
                }
                _ => return,
            };
        });
        match result {
            (Some(object), Some(point), Some(_distance)) => {
                let color: Color = object.color();
                let mut tmp: (f32, f32, f32) =
                    (color.red as f32, color.green as f32, color.blue as f32);
                // reflect
                let normal: Vector = object.normal(&point);
                let incoming: Vector = light.direction.clone();
                let projection: f32 = incoming.clone() * normal.clone();
                if projection < 0.0 {
                    let reflect = Light {
                        start: point,
                        direction: (incoming - 2.0 * projection * normal).normalvec(),
                    };
                    let r = self.trace(reflect, object.ior(), depth + 1);
                    tmp.0 += r.0;
                    tmp.1 += r.1;
                    tmp.2 += r.2;
                }
                // refract
                tmp.0 *= weight;
                tmp.1 *= weight;
                tmp.2 *= weight;
                tmp
            }
            _ => (0.0, 0.0, 0.0),
        }
    }

    pub fn jittered_sampling(&self, x: u32, y: u32) -> Pixel {
        let mut rng = SmallRng::from_entropy();
        let point: Point = Point {
            x: x as f32 / self.size as f32,
            y: y as f32 / self.size as f32,
        };
        let color = (0..self.config.n_sampling)
            .map(|x| {
                let theta: f32 =
                    PI * 2_f32 * (x as f32 + rng.gen::<f32>()) / self.config.n_sampling as f32;
                let light: Light = Light::from_theta(&point, theta);
                let c: (f32, f32, f32) = self.trace(light, 1.0, 0);
                Color::new(c.0 as u32, c.1 as u32, c.2 as u32)
            })
            .collect::<Vec<Color>>()
            .avg();
        Pixel {
            x: x,
            y: y,
            color: color,
        }
    }

    pub fn render(&self) -> RgbImage {
        let mut img: RgbImage = ImageBuffer::new(self.size, self.size);
        let start = SystemTime::now();
        let result: Vec<Pixel> = (0..self.size * self.size)
            .into_par_iter()
            .map(|index: u32| self.jittered_sampling(index % self.size, index / self.size))
            .collect();
        let stop = SystemTime::now();
        result
            .into_iter()
            .for_each(|p| img.put_pixel(p.x as u32, p.y as u32, p.color.rgb()));
        match stop.duration_since(start) {
            Ok(d) => println!(
                "rendering cost {:.6}s",
                d.as_secs() as f64 + d.subsec_nanos() as f64 * 1e-9
            ),
            Err(e) => println!("rendering error: {}", e.description()),
        }
        img
    }
}
