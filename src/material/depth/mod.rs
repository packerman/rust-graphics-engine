use web_sys::WebGl2RenderingContext;

use crate::core::{
    material::{GenericMaterial, Source},
    program::{Program, UpdateProgramUniforms},
};

#[derive(Debug, Clone)]
pub struct DepthMaterial;

impl UpdateProgramUniforms for DepthMaterial {
    fn update_program_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {}
}

impl GenericMaterial for DepthMaterial {
    fn vertex_shader(&self) -> Source<'_> {
        include_str!("vertex.glsl").into()
    }

    fn fragment_shader(&self) -> Source<'_> {
        include_str!("fragment.glsl").into()
    }

    fn preferred_mode(&self) -> Option<u32> {
        Some(WebGl2RenderingContext::TRIANGLES)
    }
}
