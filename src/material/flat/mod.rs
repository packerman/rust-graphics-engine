use std::rc::Rc;

use anyhow::Result;
use web_sys::WebGl2RenderingContext;

use crate::{
    base::{
        color::{self, Color},
        convert::FromWithContext,
    },
    classic::texture::Sampler2D,
    core::{
        material::{GenericMaterial, Material, Source},
        program::{self, Program, UpdateProgramUniforms, UpdateUniform},
    },
};

#[derive(Debug)]
pub struct FlatMaterial {
    pub double_side: bool,
    pub texture: Option<Sampler2D>,
    pub ambient: Color,
    pub diffuse: Color,
}

impl FlatMaterial {
    fn update_struct_uniform(
        &self,
        context: &WebGl2RenderingContext,
        name: &str,
        program: &Program,
    ) {
        self.ambient
            .update_uniform(context, &program::join_name(name, "ambient"), program);
        self.diffuse
            .update_uniform(context, &program::join_name(name, "diffuse"), program);
        if let Some(texture) = &self.texture {
            texture.update_uniform(context, &program::join_name(name, "texture0"), program);
        }
        self.texture.is_some().update_uniform(
            context,
            &program::join_name(name, "useTexture"),
            program,
        );
    }
}

impl Default for FlatMaterial {
    fn default() -> Self {
        Self {
            double_side: true,
            texture: None,
            ambient: color::black(),
            diffuse: color::white(),
        }
    }
}

impl UpdateProgramUniforms for FlatMaterial {
    fn update_program_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {
        self.update_struct_uniform(context, "material", program);
    }
}

impl GenericMaterial for FlatMaterial {
    fn vertex_shader(&self) -> Source<'_> {
        include_str!("vertex.glsl").into()
    }

    fn fragment_shader(&self) -> Source<'_> {
        include_str!("fragment.glsl").into()
    }

    fn double_sided(&self) -> bool {
        self.double_side
    }
}

pub fn create(
    context: &WebGl2RenderingContext,
    flat_material: FlatMaterial,
) -> Result<Rc<Material>> {
    <Rc<Material>>::from_with_context(context, flat_material)
}
