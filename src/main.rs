extern crate image;
extern crate rayon;
extern crate rand;

use image::ImageBuffer;
use image::Rgb;
use rayon::prelude::*;

mod chapter1;
mod chapter2;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;

use std::time::SystemTime;
use std::error::Error;
fn gen(fun: fn(f32, f32, Vec<u32>) -> Rgb<u8>, path: &str, param: Vec<u32>) -> String {
    let mut img: image::RgbImage = ImageBuffer::new(WIDTH, HEIGHT);
    let start = SystemTime::now();
    let result: Vec<(u32, u32, Rgb<u8>)> = (0..WIDTH * HEIGHT)
        .into_par_iter()
        .map(|point: u32| {
            let x = point / HEIGHT;
            let y = point % HEIGHT;
            let xp = x as f32 / WIDTH as f32;
            let yp = y as f32 / HEIGHT as f32;
            (x, y, fun(xp, yp, param.clone()))
        })
        .collect();
    let stop = SystemTime::now();
    for p in result {
        img.put_pixel(p.0, p.1, p.2);
    }
    match img.save(path) {
        Ok(()) => match stop.duration_since(start) {
            Ok(d) => format!("cost: {:.6} s", d.as_secs() as f64 + d.subsec_nanos() as f64 * 1e-9),
            Err(e) => format!("error: {}", e.description())
        },
        Err(e) => format!("error: {}", e.description()),
    }
}

fn main() {
    // chapter1::run();
    chapter2::run();
}
