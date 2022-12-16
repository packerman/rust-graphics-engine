use glm::Vec4;
use web_sys::WebGl2RenderingContext;

use super::program::{Program, UpdateUniform, UpdateUniforms};

#[derive(Debug)]
pub struct TestMaterial {
    pub base_color_factor: Vec4,
}

impl UpdateUniforms for TestMaterial {
    fn update_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {
        self.base_color_factor
            .update_uniform(context, "u_BaseColorFactor", program)
    }

    fn vertex_shader(&self) -> &str {
        include_str!("test.vert")
    }

    fn fragment_shader(&self) -> &str {
        include_str!("test.frag")
    }
}
