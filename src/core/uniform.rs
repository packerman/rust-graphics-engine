use anyhow::Result;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlUniformLocation};

use super::gl::get_uniform_location;

struct UniformData(Box<dyn Fn(&WebGl2RenderingContext, &WebGlUniformLocation)>);

impl UniformData {
    fn vec3(data: [f32; 3]) -> UniformData {
        UniformData(Box::new(move |context, location| {
            context.uniform3fv_with_f32_array(Some(location), &data);
        }))
    }
}

struct Uniform {
    data: Box<UniformData>,
    location: WebGlUniformLocation,
}

impl Uniform {
    fn new_with_data(
        context: &WebGl2RenderingContext,
        data: Box<UniformData>,
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

    fn upload_data(&self, context: &WebGl2RenderingContext) {
        self.data.0(context, &self.location);
    }
}
