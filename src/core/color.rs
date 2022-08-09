use glm::Vec4;

pub type Color = Vec4;

pub fn black() -> Color {
    from_rgb(0, 0, 0)
}

pub fn red() -> Color {
    from_rgb(255, 0, 0)
}

pub fn blue() -> Color {
    from_rgb(0, 0, 255)
}

pub fn gray() -> Color {
    from_rgb(128, 128, 128)
}

fn from_rgb(red: u8, green: u8, blue: u8) -> Color {
    glm::vec4(
        red as f32 / 255.0,
        green as f32 / 255.0,
        blue as f32 / 255.0,
        1.0,
    )
}
