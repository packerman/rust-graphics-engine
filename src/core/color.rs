#![allow(dead_code)]

use nalgebra_glm::{vec4, TVec4};

pub type Color = TVec4<f32>;

fn from_rgb(red: u8, green: u8, blue: u8) -> Color {
    vec4(
        red as f32 / 255.0,
        green as f32 / 255.0,
        blue as f32 / 255.0,
        1.0,
    )
}

pub fn black() -> Color {
    vec4(0.0, 0.0, 0.0, 1.0)
}

pub fn white() -> Color {
    vec4(1.0, 1.0, 1.0, 1.0)
}

pub fn red() -> Color {
    vec4(1.0, 0.0, 0.0, 1.0)
}

pub fn lime() -> Color {
    vec4(0.0, 1.0, 0.0, 1.0)
}

pub fn blue() -> Color {
    vec4(0.0, 0.0, 1.0, 1.0)
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
