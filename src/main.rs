extern crate core;

use core::{Color, Point, Scene};
use std::error::Error;

mod circle;
use circle::Circle;

fn main() {
    let mut scene = Scene::default();

    #[cfg(debug_assertions)]
    {
        scene.size = 32;
    }
    //scene.object.push(Box::new(Circle {
    //    center: Point { x: 0.2, y: 0.2 },
    //    radius: 0.1,
    //    color: Color::new(511, 0, 0),
    //    ior: std::f32::MAX,
    //}));
    //scene.object.push(Box::new(Circle {
    //    center: Point { x: 0.8, y: 0.5 },
    //    radius: 0.1,
    //    color: Color::new(0, 0, 511),
    //    ior: std::f32::MAX,
    //}));
    //scene.object.push(Box::new(Circle {
    //    center: Point { x: 0.8, y: 0.8 },
    //    radius: 0.1,
    //    color: Color::new(0, 511, 0),
    //    ior: std::f32::MAX,
    //}));
    scene.object.push(Box::new(Circle {
        center: Point { x: 0.5, y: -9.5 },
        radius: 10.0,
        color: Color::new(0, 0, 0),
        ior: 1.0,
    }));
    scene.object.push(Box::new(Circle {
        center: Point { x: 0.5, y: 0.65 },
        radius: 0.1,
        color: Color::new(511, 511, 511),
        ior: 0.0,
    }));
    match scene.render().save("./1.png") {
        Err(e) => println!("save error: {}", e.description()),
        _ => {
            std::process::Command::new("display")
                .arg("1.png")
                .spawn()
                .expect("display error!");
        }
    }
}
