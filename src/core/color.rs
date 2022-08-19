use std::ops::{Index, IndexMut};

use glm::{Vec3, Vec4};

pub struct Color(Vec4);

impl Color {
    fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color(glm::vec4(r, g, b, a))
    }

    pub fn white() -> Color {
        Self::from_rgb(255, 255, 255)
    }

    pub fn silver() -> Color {
        Self::from_rgb(192, 192, 192)
    }

    pub fn gray() -> Color {
        Self::from_rgb(128, 128, 128)
    }

    pub fn black() -> Color {
        Self::from_rgb(0, 0, 0)
    }

    pub fn red() -> Color {
        Self::from_rgb(255, 0, 0)
    }

    pub fn maroon() -> Color {
        Self::from_rgb(128, 0, 0)
    }

    pub fn yellow() -> Color {
        Self::from_rgb(255, 255, 0)
    }

    pub fn olive() -> Color {
        Self::from_rgb(128, 128, 0)
    }

    pub fn lime() -> Color {
        Self::from_rgb(0, 255, 0)
    }

    pub fn green() -> Color {
        Self::from_rgb(0, 128, 0)
    }

    pub fn aqua() -> Color {
        Self::from_rgb(0, 255, 255)
    }

    pub fn teal() -> Color {
        Self::from_rgb(0, 128, 128)
    }

    pub fn blue() -> Color {
        Self::from_rgb(0, 0, 255)
    }

    pub fn navy() -> Color {
        Self::from_rgb(0, 0, 128)
    }

    pub fn fuchsia() -> Color {
        Self::from_rgb(255, 0, 255)
    }

    pub fn purple() -> Color {
        Self::from_rgb(128, 0, 128)
    }

    pub fn dark_orange() -> Color {
        Self::from_rgb(255, 140, 0)
    }

    pub fn blue_violet() -> Color {
        Self::from_rgb(138, 43, 226)
    }

    pub fn light_coral() -> Color {
        Self::from_rgb(240, 128, 128)
    }

    pub fn light_green() -> Color {
        Self::from_rgb(144, 238, 144)
    }

    pub fn medium_slate_blue() -> Color {
        Self::from_rgb(123, 104, 238)
    }

    pub fn to_rgb_vec(&self) -> Vec<f32> {
        vec![self.0.x, self.0.y, self.0.z]
    }

    fn from_rgb(red: u8, green: u8, blue: u8) -> Color {
        Self::new(
            f32::from(red) / 255.0,
            f32::from(green) / 255.0,
            f32::from(blue) / 255.0,
            1.0,
        )
    }
}

impl From<Color> for [f32; 3] {
    fn from(color: Color) -> Self {
        [color.0.x, color.0.y, color.0.z]
    }
}

impl From<Color> for Vec3 {
    fn from(color: Color) -> Self {
        glm::vec4_to_vec3(&color.0)
    }
}

impl Index<usize> for Color {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl IndexMut<usize> for Color {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}
