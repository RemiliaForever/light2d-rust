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
        let sd = circle_sdf(ox + dx * t, oy + dy * t, 0.5, 0.5, 0.3);
        if sd < EPSILON {
            r = 2.0;
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

use std::f32::consts::PI;
use image::Rgb;
use rand::Rng;

pub fn uniform_sampling(x: f32, y: f32, param: Vec<u32>) -> Rgb<u8> {
    let mut sum = 0.0;
    let mut rng = rand::thread_rng();
    for _i in 0..param[0] {
        let a = PI * 2.0 * rng.gen::<f32>();
        sum += trace(x, y, a.cos(), a.sin());
    }
    let grey = (sum / param[0] as f32 * 255.0) as u8;
    Rgb([grey, grey, grey])
}

pub fn stratified_sampling(x: f32, y: f32, param: Vec<u32>) -> Rgb<u8> {
    let mut sum = 0.0;
    for i in 0..param[0] {
        let a = PI * 2.0 * i as f32 / param[0] as f32;
        sum += trace(x, y, a.cos(), a.sin());
    }
    let grey = (sum / param[0] as f32 * 255.0) as u8;
    Rgb([grey, grey, grey])
}

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
    // uniform_sampling
    println!(
        "{} => {}",
        "chapter1 uniform_sampling N=16",
        gen(uniform_sampling, "./target/chapter1_1_1.png", vec![16])
    );
    println!(
        "{} => {}",
        "chapter1 uniform_sampling N=64",
        gen(uniform_sampling, "./target/chapter1_1_2.png", vec![64])
    );
    println!(
        "{} => {}",
        "chapter1 uniform_sampling N=256",
        gen(uniform_sampling, "./target/chapter1_1_3.png", vec![256])
    );
    println!(
        "{} => {}",
        "chapter1 uniform_sampling N=1024",
        gen(uniform_sampling, "./target/chapter1_1_4.png", vec![1024])
    );
    // stratified_sampling
    println!(
        "{} => {}",
        "chapter1 stratified_sampling N=16",
        gen(stratified_sampling, "./target/chapter1_2_1.png", vec![16])
    );
    println!(
        "{} => {}",
        "chapter1 stratified_sampling N=64",
        gen(stratified_sampling, "./target/chapter1_2_2.png", vec![64])
    );
    println!(
        "{} => {}",
        "chapter1 stratified_sampling N=256",
        gen(stratified_sampling, "./target/chapter1_2_3.png", vec![256])
    );
    println!(
        "{} => {}",
        "chapter1 stratified_sampling N=1024",
        gen(stratified_sampling, "./target/chapter1_2_4.png", vec![1024])
    );
    // jittered_sampling
    println!(
        "{} => {}",
        "chapter1 jittered_sampling N=16",
        gen(jittered_sampling, "./target/chapter1_3_1.png", vec![16])
    );
    println!(
        "{} => {}",
        "chapter1 jittered_sampling N=64",
        gen(jittered_sampling, "./target/chapter1_3_2.png", vec![64])
    );
    println!(
        "{} => {}",
        "chapter1 jittered_sampling N=256",
        gen(jittered_sampling, "./target/chapter1_3_3.png", vec![256])
    );
    println!(
        "{} => {}",
        "chapter1 jittered_sampling N=1024",
        gen(jittered_sampling, "./target/chapter1_3_4.png", vec![1024])
    );
}
