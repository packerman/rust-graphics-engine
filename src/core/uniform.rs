use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use glm::{Mat4, Vec3, Vec4};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlUniformLocation};

use super::{attribute::DataType, color::Color, gl};

pub enum UniformData {
    Array3([f32; 3]),
    Vec3(Vec3),
    Vec4(Vec4),
    Color(Color),
    Mat4(Mat4),
}

pub struct Uniform {
    pub data: UniformData,
    location: WebGlUniformLocation,
}

impl Uniform {
    pub fn new_with_array3(
        context: &WebGl2RenderingContext,
        data: [f32; 3],
        program: &WebGlProgram,
        name: &str,
    ) -> Result<Self> {
        Self::new_with_data(context, UniformData::Array3(data), program, name)
    }

    pub fn new_with_color(
        context: &WebGl2RenderingContext,
        data: Color,
        program: &WebGlProgram,
        name: &str,
    ) -> Result<Self> {
        Self::new_with_data(context, UniformData::Color(data), program, name)
    }

    pub fn new_with_mat4(
        context: &WebGl2RenderingContext,
        data: Mat4,
        program: &WebGlProgram,
        name: &str,
    ) -> Result<Self> {
        Self::new_with_data(context, UniformData::Mat4(data), program, name)
    }

    fn new_with_data(
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
        match self.data {
            UniformData::Array3(data) => context.uniform1fv_with_f32_array(location, &data),
            UniformData::Vec3(data) => context.uniform3f(location, data.x, data.y, data.z),
            UniformData::Vec4(data) => context.uniform4f(location, data.x, data.y, data.z, data.w),
            UniformData::Color(data) => {
                context.uniform4f(location, data[0], data[1], data[2], data[3])
            }
            UniformData::Mat4(data) => {
                context.uniform_matrix4fv_with_f32_array(location, false, (&data).into())
            }
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
