use anyhow::Result;
use glm::{Mat4, Vec2};
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
    Array3([f32; 3]),
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

impl From<[f32; 3]> for UniformData {
    fn from(data: [f32; 3]) -> Self {
        UniformData::Array3(data)
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

#[derive(Clone)]
pub struct Uniform {
    pub data: UniformData,
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
        let uniform = Uniform { data, location };
        Ok(uniform)
    }

    pub fn upload_data(&self, context: &WebGl2RenderingContext) {
        let location = Some(&self.location);
        match &self.data {
            UniformData::Boolean(data) => context.uniform1i(location, i32::from(*data)),
            UniformData::Float(data) => context.uniform1f(location, *data),
            UniformData::Array3(data) => context.uniform1fv_with_f32_array(location, data),
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

    pub fn array3_mut(&mut self) -> Option<&mut [f32; 3]> {
        match &mut self.data {
            UniformData::Array3(data) => Some(data),
            _ => None,
        }
    }

    pub fn color_mut(&mut self) -> Option<&mut Color> {
        match &mut self.data {
            UniformData::Color(data) => Some(data),
            _ => None,
        }
    }

    pub fn mat4_mut(&mut self) -> Option<&mut Mat4> {
        match &mut self.data {
            UniformData::Mat4(data) => Some(data),
            _ => None,
        }
    }
}
