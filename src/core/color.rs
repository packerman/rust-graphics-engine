use glm::Vec4;

pub type Color = Vec4;

pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    glm::vec4(r, g, b, a)
}

pub fn rgb(r: f32, g: f32, b: f32) -> Color {
    self::rgba(r, g, b, 1.0)
}

pub fn white() -> Color {
    self::rgb_u8(255, 255, 255)
}

pub fn gray() -> Color {
    self::rgb_u8(128, 128, 128)
}

pub fn dark_slate_gray() -> Color {
    self::rgb_u8(47, 79, 79)
}

pub fn black() -> Color {
    self::rgb_u8(0, 0, 0)
}

pub fn red() -> Color {
    self::rgb_u8(255, 0, 0)
}

pub fn maroon() -> Color {
    self::rgb_u8(128, 0, 0)
}

pub fn yellow() -> Color {
    self::rgb_u8(255, 255, 0)
}

pub fn lime() -> Color {
    self::rgb_u8(0, 255, 0)
}

pub fn green() -> Color {
    self::rgb_u8(0, 128, 0)
}

pub fn aqua() -> Color {
    self::rgb_u8(0, 255, 255)
}

pub fn blue() -> Color {
    self::rgb_u8(0, 0, 255)
}

pub fn navy() -> Color {
    self::rgb_u8(0, 0, 128)
}

pub fn fuchsia() -> Color {
    self::rgb_u8(255, 0, 255)
}

pub fn light_coral() -> Color {
    self::rgb_u8(240, 128, 128)
}

pub fn light_green() -> Color {
    self::rgb_u8(144, 238, 144)
}

pub fn medium_slate_blue() -> Color {
    self::rgb_u8(123, 104, 238)
}

fn rgb_u8(red: u8, green: u8, blue: u8) -> Color {
    self::rgba(
        f32::from(red) / 255.0,
        f32::from(green) / 255.0,
        f32::from(blue) / 255.0,
        1.0,
    )
}
