use std::default::Default;
use std::error::Error;
use std::f32::consts::PI;
use std::time::SystemTime;

use image::{ImageBuffer, RgbImage};
use rand::{self, Rng};
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
    pub fn trace(&self, mut light: Light) -> Color {
        fn trace_once(mut light: Light, mut weight: f32) -> Color {
            // calc reflect
            //
            // calc refrac
            //
            Color::new(0, 0, 0)
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
        if let Some(object) = result.0 {
            object.color()
        } else {
            Color::new(0, 0, 0)
        }
    }

    pub fn jittered_sampling(&self, mut pixel: Pixel) -> Pixel {
        let mut rng = rand::thread_rng();
        let point: Point = Point {
            x: pixel.x as f32 / self.size as f32,
            y: pixel.y as f32 / self.size as f32,
        };
        pixel.color = (0..self.config.n_sampling)
            .map(|x| {
                let theta =
                    PI * 2_f32 * (x as f32 + rng.gen::<f32>()) / self.config.n_sampling as f32;
                let light = Light::from_theta(&point, theta);
                self.trace(light)
            })
            .collect::<Vec<Color>>()
            .avg();
        pixel
    }

    pub fn render(&self) -> RgbImage {
        let mut img: RgbImage = ImageBuffer::new(self.size, self.size);
        let start = SystemTime::now();
        let result: Vec<Pixel> = (0..self.size * self.size)
            .into_par_iter()
            .map(|index: u32| {
                let pixel = Pixel {
                    x: index % self.size,
                    y: index / self.size,
                    color: Color::new(0, 0, 0),
                };
                self.jittered_sampling(pixel)
            })
            .collect();
        let stop = SystemTime::now();
        for p in result {
            img.put_pixel(p.x as u32, p.y as u32, p.color.rgb());
        }
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
