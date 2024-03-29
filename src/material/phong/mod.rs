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
pub struct PhongMaterial {
    pub double_side: bool,
    pub texture: Option<Sampler2D>,
    pub ambient: Color,
    pub diffuse: Color,
    pub specular_strength: f32,
    pub shininess: f32,
    pub bump_texture: Option<Sampler2D>,
    pub bump_strength: f32,
    pub use_shadow: bool,
}

impl PhongMaterial {
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
        self.specular_strength.update_uniform(
            context,
            &program::join_name(name, "specularStrength"),
            program,
        );
        self.shininess
            .update_uniform(context, &program::join_name(name, "shininess"), program);
        self.texture
            .update_uniform(context, &program::join_name(name, "texture0"), program);
        self.texture.is_some().update_uniform(
            context,
            &program::join_name(name, "useTexture"),
            program,
        );
        self.bump_texture.update_uniform(
            context,
            &program::join_name(name, "bumpTexture"),
            program,
        );
        self.bump_texture.is_some().update_uniform(
            context,
            &program::join_name(name, "useBumpTexture"),
            program,
        );
        self.bump_strength.update_uniform(
            context,
            &program::join_name(name, "bumpStrength"),
            program,
        );
    }
}

impl Default for PhongMaterial {
    fn default() -> Self {
        Self {
            double_side: true,
            texture: None,
            ambient: color::black(),
            diffuse: color::white(),
            specular_strength: 1.0,
            shininess: 32.0,
            bump_texture: None,
            bump_strength: 1.0,
            use_shadow: false,
        }
    }
}

impl UpdateProgramUniforms for PhongMaterial {
    fn update_program_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {
        self.update_struct_uniform(context, "material", program);
        self.use_shadow
            .update_uniform(context, "useShadow", program);
    }
}

impl GenericMaterial for PhongMaterial {
    fn vertex_shader(&self) -> Source<'_> {
        include_str!("vertex.glsl").into()
    }

    fn fragment_shader(&self) -> Source<'_> {
        include_str!("fragment.glsl").into()
    }
}

pub fn create(
    context: &WebGl2RenderingContext,
    phong_material: PhongMaterial,
) -> Result<Rc<Material>> {
    <Rc<Material>>::from_with_context(context, phong_material)
}
