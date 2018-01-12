extern crate image;
extern crate rayon;
extern crate rand;

use image::ImageBuffer;
use image::Rgb;
use rayon::prelude::*;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

fn basic(x: f32, y: f32, _param: Vec<u8>) -> Rgb<u8> {
    let red = (1.0 - x) * 255.0;
    let green = x * 255.0;
    let blue = y * 255.0;
    Rgb([red as u8, green as u8, blue as u8])
}

use std::time::SystemTime;
use std::error::Error;
fn gen(fun: fn(f32, f32, Vec<u8>) -> Rgb<u8>, path: &str) -> String {
    let mut img: image::RgbImage = ImageBuffer::new(WIDTH, HEIGHT);
    let start = SystemTime::now();
    let result: Vec<(u32, u32, Rgb<u8>)> = (0..WIDTH * HEIGHT)
        .into_par_iter()
        .map(|point: u32| {
            let x = point / HEIGHT;
            let y = point % HEIGHT;
            let xp = x as f32 / WIDTH as f32;
            let yp = y as f32 / HEIGHT as f32;
            (x, y, fun(xp, yp, vec![]))
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
    println!("{} => {}", "basic", gen(basic, "./basic.png"));
}
