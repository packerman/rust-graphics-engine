#![allow(dead_code)]

use nalgebra_glm::{vec4, Vec4};

pub type Color = Vec4;

pub fn black() -> Color {
    from_rgb(0, 0, 0)
}

pub fn white() -> Color {
    from_rgb(255, 255, 255)
}

pub fn red() -> Color {
    from_rgb(255, 0, 0)
}

pub fn lime() -> Color {
    from_rgb(0, 255, 0)
}

pub fn blue() -> Color {
    from_rgb(0, 0, 255)
}

pub fn yellow() -> Color {
    from_rgb(255, 255, 0)
}

pub fn gray() -> Color {
    from_rgb(128, 128, 128)
}

pub fn green() -> Color {
    from_rgb(0, 128, 0)
}

pub fn orange() -> Color {
    from_rgb(255, 165, 0)
}

fn from_rgb(red: u8, green: u8, blue: u8) -> Color {
    vec4(
        red as f32 / 255.0,
        green as f32 / 255.0,
        blue as f32 / 255.0,
        1.0,
    )
}
