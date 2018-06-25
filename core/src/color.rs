use image::Rgb;
use std::ops::{Add, Div, Sub};

#[derive(Debug, Clone)]
pub struct Color {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Color {
    pub fn new(r: u32, g: u32, b: u32) -> Color {
        Color {
            red: r,
            green: g,
            blue: b,
        }
    }
    pub fn from_f32(r: f32, g: f32, b: f32) -> Color {
        Color {
            red: (r * 255.0) as u32,
            green: (g * 255.0) as u32,
            blue: (b * 255.0) as u32,
        }
    }
    pub fn rgb(&self) -> Rgb<u8> {
        fn to_u8(color: u32) -> u8 {
            if color > 255 {
                255
            } else {
                color as u8
            }
        }
        Rgb([to_u8(self.red), to_u8(self.green), to_u8(self.blue)])
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Sub for Color {
    type Output = Color;
    fn sub(self, other: Color) -> Color {
        fn sub_check(a: u32, b: u32) -> u32 {
            if a < b {
                0
            } else {
                a - b
            }
        }
        Color {
            red: sub_check(self.red, other.red),
            green: sub_check(self.green, other.green),
            blue: sub_check(self.blue, other.blue),
        }
    }
}

impl Div<u32> for Color {
    type Output = Color;
    fn div(self, other: u32) -> Color {
        Color {
            red: self.red / other,
            green: self.green / other,
            blue: self.blue / other,
        }
    }
}

pub trait ColorAvg {
    fn avg(&self) -> Color;
}

impl ColorAvg for Vec<Color> {
    fn avg(&self) -> Color {
        let mut sum = (0, 0, 0);
        for c in self {
            sum.0 += c.red;
            sum.1 += c.green;
            sum.2 += c.blue;
        }
        let length = self.len() as u32;
        Color::new(sum.0 / length, sum.1 / length, sum.2 / length)
    }
}
