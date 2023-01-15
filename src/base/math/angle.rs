use std::{
    f32::consts::{FRAC_PI_2, PI, TAU},
    ops::{Div, Mul, Neg},
};

#[derive(Debug, Clone, Copy)]
pub struct Angle {
    radians: f32,
}

impl Angle {
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
