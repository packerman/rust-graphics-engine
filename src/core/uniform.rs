use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use glm::{Mat4, Vec3, Vec4};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlUniformLocation};

use super::{color::Color, gl};

pub struct Uniform<'a> {
    pub data: Rc<RefCell<dyn UniformData + 'a>>,
    location: WebGlUniformLocation,
}

impl<'a> Uniform<'a> {
    pub fn new_with_data<T: UniformData + 'a>(
        context: &WebGl2RenderingContext,
        data: Rc<RefCell<T>>,
        program: &WebGlProgram,
        name: &str,
    ) -> Result<Uniform<'a>> {
        let location = gl::get_uniform_location(context, program, name)?;
        let uniform = Uniform { data, location };
        Ok(uniform)
    }

    pub fn upload_data(&self, context: &WebGl2RenderingContext) {
        self.data.borrow().upload_data(context, &self.location);
    }
}

pub trait UniformData {
    fn upload_data(&self, context: &WebGl2RenderingContext, location: &WebGlUniformLocation);
}

impl UniformData for [f32; 3] {
    fn upload_data(&self, context: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        context.uniform3fv_with_f32_array(Some(location), self);
    }
}

impl UniformData for Vec3 {
    fn upload_data(&self, context: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        context.uniform3f(Some(location), self.x, self.y, self.z);
    }
}

impl UniformData for Vec4 {
    fn upload_data(&self, context: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        context.uniform4f(Some(location), self.x, self.y, self.z, self.w);
    }
}

impl UniformData for Color {
    fn upload_data(&self, context: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        context.uniform4f(Some(location), self[0], self[1], self[2], self[3]);
    }
}

impl UniformData for Mat4 {
    fn upload_data(&self, context: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        context.uniform_matrix4fv_with_f32_array(Some(location), false, self.into());
    }
}
