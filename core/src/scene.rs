use std::f32::consts::PI;
use std::time::SystemTime;
use std::error::Error;
use std::default::Default;

use rand;
use rand::Rng;
use image::RgbImage;
use image::ImageBuffer;
use rayon::prelude::*;

use super::*;

pub struct Scene {
    pub config: SceneConfig,
    pub width: u32,
    pub height: u32,
    pub object: Vec<Box<Object + Sync>>,
}
impl Default for Scene {
    fn default() -> Scene {
        Scene {
            config: SceneConfig::default(),
            width: 1024,
            height: 1024,
            object: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct SceneConfig {
    pub max_step: u32,
    pub max_distance: f32,
    pub epsilon: f32,
    pub n_sampling: u32,
}
impl Default for SceneConfig {
    fn default() -> SceneConfig {
        SceneConfig {
            max_step: 10,
            max_distance: 2.0,
            epsilon: 1e-6,
            n_sampling: 512,
        }
    }
}

impl Scene {
    pub fn trace(&self, mut light: Light) -> Color {
        let mut distance = 0_f32;
        let mut result = Color::new(0, 0, 0);
        for _i in 0..self.config.max_step {
            if distance >= self.config.max_distance {
                break;
            }
            let mut sd = std::f32::MAX;
            for _object in &self.object {
                let p_sd = _object.sdf(&light);
                if sd > p_sd {
                    sd = p_sd;
                    if p_sd < self.config.epsilon {
                        result = _object.color();
                        break;
                    }
                }
            }
            distance += sd;
            light.trace(sd);
        }
        result
    }
    pub fn jittered_sampling(&self, mut pixel: Pixel) -> Pixel {
        let mut sum = Color::new(0, 0, 0);
        let mut rng = rand::thread_rng();
        let point: Point = Point {
            x: pixel.x as f32 / self.width as f32,
            y: pixel.y as f32 / self.height as f32,
        };
        for i in 0..self.config.n_sampling {
            let theta = PI * 2_f32 * (i as f32 + rng.gen::<f32>()) / self.config.n_sampling as f32;
            let light = Light {
                start: point.clone(),
                direction: Vector::from_theta(theta),
            };
            sum = sum + self.trace(light);
        }
        pixel.color = sum / self.config.n_sampling;
        pixel
    }

    pub fn render(&self) -> RgbImage {
        let mut img: RgbImage = ImageBuffer::new(self.width, self.height);
        let start = SystemTime::now();
        let result: Vec<Pixel> = (0..self.width * self.height)
            .into_par_iter()
            .map(|index: u32| {
                let pixel = Pixel {
                    x: index % self.width,
                    y: index / self.width,
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
            Ok(d) => {
                println!(
                    "rendering cost {:.6}s",
                    d.as_secs() as f64 + d.subsec_nanos() as f64 * 1e-9
                )
            }
            Err(e) => println!("rendering error: {}", e.description()),
        }
        img
    }
}
