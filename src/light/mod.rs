use std::collections::HashMap;

use glm::Vec3;

use crate::core::{color::Color, uniform::data::Data};

#[derive(Debug, Clone, Copy)]
pub enum LightType {
    Directional { direction: Vec3 },
    Point { position: Vec3 },
}

#[derive(Debug, Clone, Copy)]
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
        let mut members = HashMap::<&str, Data>::new();
        let light_type: i32;
        match light.light_type {
            LightType::Directional { direction } => {
                light_type = 1;
                members.insert("direction", Data::from(direction));
            }
            LightType::Point { position } => {
                light_type = 2;
                members.insert("position", Data::from(position));
            }
        }
        members.insert("lightType", Data::from(light_type));
        members.insert("color", Data::from(light.color));
        members.insert("attenuation", Data::from(light.attenuation));
        Self::from(members)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Attenuation(f32, f32, f32);

impl Default for Attenuation {
    fn default() -> Self {
        Self(1.0, 0.0, 0.0)
    }
}

impl From<Attenuation> for Data {
    fn from(attenuation: Attenuation) -> Self {
        Data::from(glm::vec3(attenuation.0, attenuation.1, attenuation.2))
    }
}
