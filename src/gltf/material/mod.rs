use glm::{Vec3, Vec4};
use web_sys::WebGl2RenderingContext;

use crate::core::color;

use super::program::{Program, UpdateUniform, UpdateUniforms};

#[derive(Debug)]
pub struct TestMaterial {
    pub base_color_factor: Vec4,
    pub min_factor: f32,
    pub light: Vec3,
}

impl Default for TestMaterial {
    fn default() -> Self {
        Self {
            base_color_factor: color::white(),
            min_factor: 0.2,
            light: glm::vec3(1.0, 1.0, 1.0),
        }
    }
}

impl UpdateUniforms for TestMaterial {
    fn update_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {
        self.base_color_factor
            .update_uniform(context, "u_BaseColorFactor", program);
        self.min_factor
            .update_uniform(context, "u_MinFactor", program);
        self.light.update_uniform(context, "u_Light", program);
    }

    fn vertex_shader(&self) -> &str {
        include_str!("test.vert")
    }

    fn fragment_shader(&self) -> &str {
        include_str!("test.frag")
    }
}
