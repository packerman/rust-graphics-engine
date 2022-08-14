use std::ops::{Mul, Neg};

use glm::Mat4;

pub fn identity() -> Mat4 {
    glm::identity()
}

pub fn translation(x: f32, y: f32, z: f32) -> Mat4 {
    glm::translation(&glm::vec3(x, y, z))
}

#[derive(Clone, Copy)]
pub struct Angle(f32);

impl Angle {
    pub fn from_degrees(degrees: f32) -> Self {
        Self(degrees.to_radians())
    }

    pub fn from_radians(radians: f32) -> Self {
        Self(radians)
    }

    pub fn to_radians(&self) -> f32 {
        self.0
    }
}

impl Mul<f32> for Angle {
    type Output = Self;

    fn mul(self, s: f32) -> Self::Output {
        Self(self.0 * s)
    }
}

impl Neg for Angle {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

#[allow(dead_code)]
pub fn rotation_x(angle: Angle) -> Mat4 {
    glm::rotation(angle.to_radians(), &glm::vec3(1.0, 0.0, 0.0))
}

#[allow(dead_code)]
pub fn rotation_y(angle: Angle) -> Mat4 {
    glm::rotation(angle.to_radians(), &glm::vec3(0.0, 1.0, 0.0))
}

pub fn rotation_z(angle: Angle) -> Mat4 {
    glm::rotation(angle.to_radians(), &glm::vec3(0.0, 0.0, 1.0))
}

#[allow(dead_code)]
pub fn scale(s: f32) -> Mat4 {
    glm::scaling(&glm::vec3(s, s, s))
}

pub struct Perspective {
    pub aspect_ratio: f32,
    pub angle_of_view: Angle,
    pub near: f32,
    pub far: f32,
}

impl Perspective {
    pub fn set_aspect_ratio<T>(&mut self, width: T, height: T)
    where
        T: Into<f32>,
    {
        self.aspect_ratio = width.into() / height.into()
    }
}

impl Default for Perspective {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            angle_of_view: Angle::from_degrees(60.0),
            near: 0.1,
            far: 1000.0,
        }
    }
}

impl From<Perspective> for Mat4 {
    fn from(perspective: Perspective) -> Self {
        glm::perspective(
            perspective.aspect_ratio,
            perspective.angle_of_view.to_radians(),
            perspective.near,
            perspective.far,
        )
    }
}
