extern crate image;
extern crate rand;
extern crate rayon;

use std::io::Error;

use image::ImageBuffer;
use image::Rgb;
use rand::Rng;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;


fn save(path: &str, data: [[[u8; 3]; HEIGHT]; WIDTH]) -> Result<(), Error> {
    let mut img: image::RgbImage = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            img.put_pixel(x as u32, y as u32, Rgb(data[x][y]));
        }
    }
    img.save(path)
}



fn main() {
    let mut data = [[[0; 3]; HEIGHT]; WIDTH];
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            data[x][y][0] = 255u8;
            data[x][y][1] = (x as f64 / WIDTH as f64 * 255 as f64) as u8;
            data[x][y][2] = (y as f64 / HEIGHT as f64 * 255 as f64) as u8;
        }
    }
    let _ = save("./output.png", data);
}
