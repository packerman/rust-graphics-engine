use web_sys::WebGl2RenderingContext;

use crate::core::{
    material::GenericMaterial,
    program::{Program, UpdateProgramUniforms},
};

#[derive(Debug)]
struct DepthMaterial;

impl UpdateProgramUniforms for DepthMaterial {
    fn update_program_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {}
}

impl GenericMaterial for DepthMaterial {
    fn vertex_shader(&self) -> &str {
        include_str!("vertex.glsl")
    }

    fn fragment_shader(&self) -> &str {
        include_str!("fragment.glsl")
    }

    fn preferred_mode(&self) -> Option<u32> {
        Some(WebGl2RenderingContext::TRIANGLES)
    }
}
