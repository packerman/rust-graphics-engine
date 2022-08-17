use anyhow::Result;
use glm::{Mat4, Vec3, Vec4};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlUniformLocation};

use super::{color::Color, gl};

pub struct Uniform<T> {
    pub data: T,
    location: WebGlUniformLocation,
}

impl<T: UploadData> Uniform<T> {
    pub fn new_with_data(
        context: &WebGl2RenderingContext,
        data: T,
        program: &WebGlProgram,
        name: &str,
    ) -> Result<Uniform<T>> {
        let location = gl::get_uniform_location(context, program, name)?;
        let uniform = Uniform { data, location };
        Ok(uniform)
    }

    #[allow(dead_code)]
    pub fn locate_variable(
        &mut self,
        context: &WebGl2RenderingContext,
        program: &WebGlProgram,
        name: &str,
    ) -> Result<()> {
        self.location = gl::get_uniform_location(context, program, name)?;
        Ok(())
    }

    pub fn upload_data(&self, context: &WebGl2RenderingContext) {
        self.data.upload_data(context, &self.location);
    }
}

pub trait UploadData {
    fn upload_data(&self, context: &WebGl2RenderingContext, location: &WebGlUniformLocation);
}

impl UploadData for [f32; 3] {
    fn upload_data(&self, context: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        context.uniform3fv_with_f32_array(Some(location), self);
    }
}

impl UploadData for Vec3 {
    fn upload_data(&self, context: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        context.uniform3f(Some(location), self.x, self.y, self.z);
    }
}

impl UploadData for Vec4 {
    fn upload_data(&self, context: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        context.uniform4f(Some(location), self.x, self.y, self.z, self.w);
    }
}

impl UploadData for Color {
    fn upload_data(&self, context: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        context.uniform4f(Some(location), self[0], self[1], self[2], self[3]);
    }
}

impl UploadData for Mat4 {
    fn upload_data(&self, context: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        context.uniform_matrix4fv_with_f32_array(Some(location), false, self.into());
    }
}
