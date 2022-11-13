use glm::Vec3;

use crate::core::math::{matrix::Ortographic, resolution::Resolution};

use super::Light;

#[derive(Debug, Clone, Copy)]
pub struct CameraBounds {
    pub min: Vec3,
    pub max: Vec3,
}

impl Default for CameraBounds {
    fn default() -> Self {
        Self {
            min: glm::vec3(-5.0, -5.0, 0.0),
            max: glm::vec3(5.0, 5.0, 20.0),
        }
    }
}

impl From<CameraBounds> for Ortographic {
    fn from(camera_bounds: CameraBounds) -> Self {
        Ortographic {
            left: camera_bounds.min.x,
            right: camera_bounds.max.x,
            bottom: camera_bounds.min.y,
            top: camera_bounds.max.x,
            near: camera_bounds.min.z,
            far: camera_bounds.max.z,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ShadowOptions {
    strength: f32,
    camera_bounds: CameraBounds,
    bias: f32,
}

impl Default for ShadowOptions {
    fn default() -> Self {
        Self {
            strength: 0.5,
            camera_bounds: Default::default(),
            bias: 0.01,
        }
    }
}

pub struct Shadow<'a> {
    light_source: &'a Light,
    resolution: Resolution,
    options: ShadowOptions,
}

impl<'a> Shadow<'a> {
    pub fn new(light_source: &'a Light, resolution: Resolution, options: ShadowOptions) -> Self {
        Self {
            light_source,
            resolution,
            options,
        }
    }
}
