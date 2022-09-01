use std::{
    f32::consts::{FRAC_PI_2, PI, TAU},
    ops::{Div, Mul, Neg},
};

use glm::Mat4;

pub fn identity() -> Mat4 {
    glm::identity()
}

pub fn translation(x: f32, y: f32, z: f32) -> Mat4 {
    glm::translation(&glm::vec3(x, y, z))
}

#[derive(Clone, Copy)]
pub struct Angle {
    radians: f32,
}

impl Angle {
    pub const ZERO: Angle = Angle::from_radians(0.0);
    pub const RIGHT: Angle = Angle::from_radians(FRAC_PI_2);
    pub const STRAIGHT: Angle = Angle::from_radians(PI);
    pub const COMPLETE: Angle = Angle::from_radians(TAU);

    pub fn from_degrees(degrees: f32) -> Self {
        Self {
            radians: degrees.to_radians(),
        }
    }

    pub const fn from_radians(radians: f32) -> Self {
        Self { radians }
    }

    pub fn to_radians(self) -> f32 {
        self.radians
    }

    pub fn sin(&self) -> f32 {
        self.radians.sin()
    }

    pub fn cos(&self) -> f32 {
        self.radians.cos()
    }
}

impl Mul<f32> for Angle {
    type Output = Self;

    fn mul(self, s: f32) -> Self::Output {
        Self::from_radians(self.radians * s)
    }
}

impl Div<f32> for Angle {
    type Output = Self;

    fn div(self, s: f32) -> Self::Output {
        Self::from_radians(self.radians / s)
    }
}

impl Neg for Angle {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from_radians(-self.radians)
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

#[derive(Clone, Copy)]
pub struct Perspective {
    pub aspect_ratio: f32,
    pub angle_of_view: Angle,
    pub near: f32,
    pub far: f32,
}

impl Perspective {
    pub fn set_aspect_ratio(&mut self, width: u32, height: u32) {
        self.aspect_ratio = (width as f32) / (height as f32);
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
