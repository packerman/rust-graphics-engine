use glm::Vec3;

use crate::core::{color::Color, uniform::data::Data};

pub enum LightType {
    Directional { direction: Vec3 },
    Point { position: Vec3 },
}

pub struct Light {
    light_type: LightType,
    color: Color,
    attenuation: Attenuation,
}

impl Light {
    pub fn point() -> Self {
        Self {
            light_type: LightType::Point {
                position: glm::vec3(0.0, 0.0, 0.0),
            },
            color: Color::white(),
            attenuation: Attenuation(1.0, 0.0, 0.1),
        }
    }
}

impl Default for Light {
    fn default() -> Self {
        Self {
            light_type: LightType::Directional {
                direction: glm::vec3(0.0, -1.0, 0.0),
            },
            color: Color::white(),
            attenuation: Default::default(),
        }
    }
}

impl From<Light> for Data {
    fn from(light: Light) -> Self {
        todo!()
    }
}

pub struct Attenuation(f32, f32, f32);

impl Default for Attenuation {
    fn default() -> Self {
        Self(1.0, 0.0, 0.0)
    }
}

impl From<Attenuation> for Vec3 {
    fn from(attenuation: Attenuation) -> Self {
        glm::vec3(attenuation.0, attenuation.1, attenuation.2)
    }
}
