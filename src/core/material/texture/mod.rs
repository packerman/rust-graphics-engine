use glm::Vec2;

use crate::core::color::Color;

struct TextureMaterial {
    base_color: Color,
    repeat_uv: Vec2,
    offset_uv: Vec2,
    double_side: bool,
    line_width: f32,
}

impl Default for TextureMaterial {
    fn default() -> Self {
        Self {
            base_color: Color::white(),
            repeat_uv: glm::vec2(1.0, 1.0),
            offset_uv: glm::vec2(0.0, 0.0),
            double_side: true,
            line_width: 1.0,
        }
    }
}
