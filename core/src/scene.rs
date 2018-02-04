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

#[derive(Debug)]
pub struct Scene {
    pub config: SceneConfig,
    pub width: u32,
    pub height: u32,
}
impl Default for Scene {
    fn default() -> Scene {
        Scene {
            config: SceneConfig::default(),
            width: 1024,
            height: 1024,
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
            n_sampling: 256,
        }
    }
}

impl Scene {
    pub fn jittered_sampling(&self, point: Point) -> Pixel {
        let mut sum = 0.0;
        let mut rng = rand::thread_rng();
        for i in 0..self.config.n_sampling {
            let a = PI * 2.0 * (i as f32 + rng.gen::<f32>()) / self.config.n_sampling as f32;
            sum += Light::trace().red as f32;
        }
        let grey = (sum / self.config.n_sampling as f32 * 255.0) as u8;
        Pixel {
            point: point,
            color: Color {
                red: grey,
                green: grey,
                blue: grey,
            },
        }
    }

    pub fn render(&self) -> RgbImage {
        let mut img: RgbImage = ImageBuffer::new(self.width, self.height);
        let start = SystemTime::now();
        let result: Vec<Pixel> = (0..self.width * self.height)
            .into_par_iter()
            .map(|index: u32| {
                let x = index / self.width;
                let y = index % self.height;
                let point = Point {
                    x: x as f32,
                    y: y as f32,
                };
                self.jittered_sampling(point)
            })
            .collect();
        let stop = SystemTime::now();
        for p in result {
            img.put_pixel(p.point.x as u32, p.point.y as u32, p.color.rgb());
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
