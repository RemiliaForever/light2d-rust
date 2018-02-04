use std::ops::{Add, Sub};
use image::Rgb;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            red: r,
            green: g,
            blue: b,
        }
    }
    pub fn rgb(&self) -> Rgb<u8> {
        Rgb([self.red, self.green, self.blue])
    }
}
impl Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        fn add_check(a: u8, b: u8) -> u8 {
            if b > 255 - a { 255 } else { a + b }
        }
        Color {
            red: add_check(self.red, other.red),
            green: add_check(self.green, other.green),
            blue: add_check(self.blue, other.blue),
        }
    }
}
impl Sub for Color {
    type Output = Color;
    fn sub(self, other: Color) -> Color {
        fn sub_check(a: u8, b: u8) -> u8 {
            if a < b { 0 } else { a - b }
        }
        Color {
            red: sub_check(self.red, other.red),
            green: sub_check(self.green, other.green),
            blue: sub_check(self.blue, other.blue),
        }
    }
}
