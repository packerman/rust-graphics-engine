use std::{
    cell::{RefCell, RefMut},
    convert::TryFrom,
};

use anyhow::{anyhow, Result};
use glm::{Mat4, Vec2, Vec3};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlUniformLocation};

use super::{
    color::Color,
    gl,
    texture::{Texture, TextureUnit},
};

#[derive(Clone)]
pub enum UniformData {
    Boolean(bool),
    Float(f32),
    Vec3(Vec3),
    Color(Color),
    Mat4(Mat4),
    Sampler2D { texture: Texture, unit: TextureUnit },
    Vec2(Vec2),
}

impl UniformData {
    pub fn sampler2d(texture: Texture, unit: TextureUnit) -> Self {
        UniformData::Sampler2D { texture, unit }
    }
}

impl From<bool> for UniformData {
    fn from(data: bool) -> Self {
        UniformData::Boolean(data)
    }
}

impl From<f32> for UniformData {
    fn from(data: f32) -> Self {
        UniformData::Float(data)
    }
}

impl From<Vec3> for UniformData {
    fn from(data: Vec3) -> Self {
        UniformData::Vec3(data)
    }
}

impl From<Color> for UniformData {
    fn from(data: Color) -> Self {
        UniformData::Color(data)
    }
}

impl From<Mat4> for UniformData {
    fn from(data: Mat4) -> Self {
        UniformData::Mat4(data)
    }
}

impl From<Vec2> for UniformData {
    fn from(data: Vec2) -> Self {
        UniformData::Vec2(data)
    }
}

impl<'a> TryFrom<&'a mut UniformData> for &'a mut Vec3 {
    type Error = anyhow::Error;

    fn try_from(value: &'a mut UniformData) -> Result<Self> {
        match value {
            UniformData::Vec3(data) => Ok(data),
            _ => Err(anyhow!("Cannot convert uniform to vec3")),
        }
    }
}

impl<'a> TryFrom<&'a mut UniformData> for &'a mut Color {
    type Error = anyhow::Error;

    fn try_from(value: &'a mut UniformData) -> Result<Self> {
        match value {
            UniformData::Color(data) => Ok(data),
            _ => Err(anyhow!("Cannot convert uniform to color")),
        }
    }
}

impl<'a> TryFrom<&'a mut UniformData> for &'a mut Mat4 {
    type Error = anyhow::Error;

    fn try_from(value: &'a mut UniformData) -> Result<Self> {
        match value {
            UniformData::Mat4(data) => Ok(data),
            _ => Err(anyhow!("Cannot convert uniform to mat4")),
        }
    }
}

impl<'a> TryFrom<&'a mut UniformData> for &'a mut f32 {
    type Error = anyhow::Error;

    fn try_from(value: &'a mut UniformData) -> Result<Self> {
        match value {
            UniformData::Float(data) => Ok(data),
            _ => Err(anyhow!("Cannot convert uniform to f32")),
        }
    }
}

#[derive(Clone)]
pub struct Uniform {
    data: RefCell<UniformData>,
    location: WebGlUniformLocation,
}

impl Uniform {
    pub fn new_with_data(
        context: &WebGl2RenderingContext,
        data: UniformData,
        program: &WebGlProgram,
        name: &str,
    ) -> Result<Self> {
        let location = gl::get_uniform_location(context, program, name)?;
        let uniform = Uniform {
            data: RefCell::new(data),
            location,
        };
        Ok(uniform)
    }

    pub fn upload_data(&self, context: &WebGl2RenderingContext) {
        let location = Some(&self.location);
        match &*self.data.borrow() {
            UniformData::Boolean(data) => context.uniform1i(location, i32::from(*data)),
            UniformData::Float(data) => context.uniform1f(location, *data),
            UniformData::Vec3(data) => context.uniform3f(location, data.x, data.y, data.z),
            UniformData::Color(data) => {
                context.uniform4f(location, data[0], data[1], data[2], data[3])
            }
            UniformData::Mat4(data) => {
                context.uniform_matrix4fv_with_f32_array(location, false, data.into())
            }
            UniformData::Sampler2D { texture, unit } => {
                unit.upload_data(context, location, texture.texture())
            }
            UniformData::Vec2(data) => context.uniform2f(location, data.x, data.y),
        }
    }

    pub fn data_ref_mut(&self) -> RefMut<UniformData> {
        self.data.borrow_mut()
    }
}
