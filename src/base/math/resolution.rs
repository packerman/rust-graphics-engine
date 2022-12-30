use glm::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Resolution {
    pub width: i32,
    pub height: i32,
}

impl Resolution {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
}

impl From<Resolution> for Vec2 {
    fn from(resolution: Resolution) -> Self {
        glm::vec2(resolution.width as f32, resolution.height as f32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ratio_works() {
        let resolution = Resolution::new(800, 600);
        assert_eq!(resolution.aspect_ratio(), 1.3333334);
    }
}
