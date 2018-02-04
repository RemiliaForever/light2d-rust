extern crate rand;

const MAX_STEP: u32 = 10;
const MAX_DISTANCE: f32 = 2.0;
const EPSILON: f32 = 1e-6;

fn trace(ox: f32, oy: f32, dx: f32, dy: f32) -> f32 {
    let mut t = 0.0;
    let mut r = 0.0;
    for _i in 0..MAX_STEP {
        if t >= MAX_DISTANCE {
            break;
        }
        let (sd, em) = scene(ox + dx * t, oy + dy * t);
        if sd < EPSILON {
            r = em;
            break;
        }
        t += sd;
    }
    r
}

fn circle_sdf(x: f32, y: f32, cx: f32, cy: f32, r: f32) -> f32 {
    let ux = x - cx;
    let uy = y - cy;
    (ux * ux + uy * uy).sqrt() - r
}
fn scene(x: f32, y: f32) -> (f32, f32) {
    fn union(a: (f32, f32), b: (f32, f32)) -> (f32, f32) {
        if a.0 < b.0 { a } else { b }
    }
    fn intersec(a: (f32, f32), b: (f32, f32)) -> (f32, f32) {
        let mut r = if a.0 < b.0 { a } else { b };
        r.0 = if a.0 < b.0 { b.0 } else { a.0 };
        r
    }
    fn subtrac(a: (f32, f32), b: (f32, f32)) -> (f32, f32) {
        let mut r = a;
        r.0 = if a.0 > -b.0 { a.0 } else { b.0 };
        r
    } // let r1 = (circle_sdf(x, y, 0.3, 0.3, 0.1), 2.0);
    // let r2 = (circle_sdf(x, y, 0.3, 0.7, 0.05), 0.8);
    // let r3 = (circle_sdf(x, y, 0.7, 0.5, 0.1), 0.0);
    // union(union(r1, r2), r3)
    let r1 = (circle_sdf(x, y, 0.4, 0.5, 0.2), 1.0);
    let r2 = (circle_sdf(x, y, 0.6, 0.5, 0.2), 0.8);
    intersec(r1, r2)
}

use std::f32::consts::PI;
use image::Rgb;
use rand::Rng;

pub fn jittered_sampling(x: f32, y: f32, param: Vec<u32>) -> Rgb<u8> {
    let mut sum = 0.0;
    let mut rng = rand::thread_rng();
    for i in 0..param[0] {
        let a = PI * 2.0 * (i as f32 + rng.gen::<f32>()) / param[0] as f32;
        sum += trace(x, y, a.cos(), a.sin());
    }
    let grey = (sum / param[0] as f32 * 255.0) as u8;
    Rgb([grey, grey, grey])
}


use super::gen;
pub fn run() {
    println!(
        "{} => {}",
        "chapter2 N=1024 section 3",
        gen(jittered_sampling, "./target/chapter2_3.png", vec![1024])
    );
}
