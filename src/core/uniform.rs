extern crate nalgebra_glm as glm;

use anyhow::Result;
use glm::{Vec3, Vec4};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlUniformLocation};

use super::gl::get_uniform_location;

pub struct Uniform<T> {
    pub data: T,
    location: WebGlUniformLocation,
}

impl<T> Uniform<T> {
    pub fn new_with_data(
        context: &WebGl2RenderingContext,
        data: T,
        program: &WebGlProgram,
        name: &str,
    ) -> Result<Uniform<T>> {
        let location = get_uniform_location(context, program, name)?;
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
        self.location = get_uniform_location(context, program, name)?;
        Ok(())
    }
}

pub trait UploadData {
    fn upload_data(&self, context: &WebGl2RenderingContext);
}

impl UploadData for Uniform<[f32; 3]> {
    fn upload_data(&self, context: &WebGl2RenderingContext) {
        context.uniform3fv_with_f32_array(Some(&self.location), &self.data);
    }
}

impl UploadData for Uniform<Vec3> {
    fn upload_data(&self, context: &WebGl2RenderingContext) {
        context.uniform3f(Some(&self.location), self.data.x, self.data.y, self.data.z);
    }
}

impl UploadData for Uniform<Vec4> {
    fn upload_data(&self, context: &WebGl2RenderingContext) {
        context.uniform4f(
            Some(&self.location),
            self.data.x,
            self.data.y,
            self.data.z,
            self.data.w,
        );
    }
}
