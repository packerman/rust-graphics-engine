use glm::{Vec3, Vec4};

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

pub fn lime() -> Color {
    from_rgb(0, 255, 0)
}

pub fn gray() -> Color {
    from_rgb(128, 128, 128)
}

pub fn yellow() -> Color {
    from_rgb(255, 255, 0)
}

pub fn dark_orange() -> Color {
    from_rgb(255, 140, 0)
}

pub fn blue_violet() -> Color {
    from_rgb(138, 43, 226)
}

pub fn to_vec3(color: &Color) -> Vec3 {
    glm::vec4_to_vec3(color)
}

pub fn to_array3(color: &Color) -> [f32; 3] {
    to_vec3(color).into()
}

fn from_rgb(red: u8, green: u8, blue: u8) -> Color {
    glm::vec4(
        red as f32 / 255.0,
        green as f32 / 255.0,
        blue as f32 / 255.0,
        1.0,
    )
}
