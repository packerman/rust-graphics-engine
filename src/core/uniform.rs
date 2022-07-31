use anyhow::Result;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlUniformLocation};

use super::gl::get_uniform_location;

pub struct UniformData(Box<dyn Fn(&WebGl2RenderingContext, &WebGlUniformLocation)>);

impl From<i32> for UniformData {
    fn from(data: i32) -> Self {
        UniformData(Box::new(move |context, location| {
            context.uniform1i(Some(location), data);
        }))
    }
}

impl From<bool> for UniformData {
    fn from(data: bool) -> Self {
        UniformData(Box::new(move |context, location| {
            context.uniform1i(Some(location), data.into());
        }))
    }
}

impl From<f32> for UniformData {
    fn from(data: f32) -> Self {
        UniformData(Box::new(move |context, location| {
            context.uniform1f(Some(location), data);
        }))
    }
}

impl From<[f32; 2]> for UniformData {
    fn from(data: [f32; 2]) -> Self {
        UniformData(Box::new(move |context, location| {
            context.uniform2fv_with_f32_array(Some(location), &data);
        }))
    }
}

impl From<[f32; 3]> for UniformData {
    fn from(data: [f32; 3]) -> Self {
        UniformData(Box::new(move |context, location| {
            context.uniform3fv_with_f32_array(Some(location), &data);
        }))
    }
}

impl From<[f32; 4]> for UniformData {
    fn from(data: [f32; 4]) -> Self {
        UniformData(Box::new(move |context, location| {
            context.uniform4fv_with_f32_array(Some(location), &data);
        }))
    }
}

pub struct Uniform {
    data: UniformData,
    location: WebGlUniformLocation,
}

impl Uniform {
    pub fn new_with_data(
        context: &WebGl2RenderingContext,
        data: UniformData,
        program: &WebGlProgram,
        name: &str,
    ) -> Result<Uniform> {
        let location = get_uniform_location(context, program, name)?;
        let uniform = Uniform { data, location };
        Ok(uniform)
    }

    fn locate_variable(
        &mut self,
        context: &WebGl2RenderingContext,
        program: &WebGlProgram,
        name: &str,
    ) -> Result<()> {
        self.location = get_uniform_location(context, program, name)?;
        Ok(())
    }

    pub fn upload_data(&self, context: &WebGl2RenderingContext) {
        self.data.0(context, &self.location);
    }
}
