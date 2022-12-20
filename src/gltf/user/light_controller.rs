use glm::Vec3;

use crate::base::input::KeyState;

#[derive(Debug, Clone)]
pub struct Properties {
    step: f32,
    key_forwards: Option<String>,
    key_backwards: Option<String>,
    key_left: Option<String>,
    key_right: Option<String>,
}

impl Default for Properties {
    fn default() -> Self {
        Self {
            step: 1.0,
            key_forwards: Some("Digit1".into()),
            key_backwards: Some("Digit2".into()),
            key_left: Some("Digit3".into()),
            key_right: Some("Digit4".into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LightController {
    alpha: f32,
    beta: f32,
    properties: Properties,
}

impl LightController {
    pub fn new(properties: Properties) -> Self {
        Self {
            alpha: 0.0,
            beta: 0.0,
            properties,
        }
    }

    pub fn get_light_direction(&self) -> Vec3 {
        let alpha = self.alpha.to_radians();
        let beta = self.beta.to_radians();
        glm::vec3(
            beta.sin(),
            -alpha.cos() * beta.cos(),
            alpha.sin() * beta.cos(),
        )
    }

    pub fn update(&mut self, key_state: &KeyState) {
        if Self::is_key_pressed(&self.properties.key_forwards, key_state) {
            self.alpha -= self.properties.step;
        }
        if Self::is_key_pressed(&self.properties.key_backwards, key_state) {
            self.alpha += self.properties.step;
        }
        if Self::is_key_pressed(&self.properties.key_left, key_state) {
            self.beta -= self.properties.step;
        }
        if Self::is_key_pressed(&self.properties.key_right, key_state) {
            self.beta += self.properties.step;
        }
    }

    fn is_key_pressed(key: &Option<String>, key_state: &KeyState) -> bool {
        key.as_ref()
            .filter(|key| key_state.is_pressed(key))
            .is_some()
    }
}
