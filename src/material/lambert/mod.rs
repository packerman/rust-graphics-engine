use web_sys::WebGl2RenderingContext;

use crate::{
    base::color::{self, Color},
    core::{
        material::GenericMaterial,
        program::{self, Program, UpdateProgramUniforms, UpdateUniform},
    },
    legacy::texture::Sampler2D,
};

#[derive(Debug, Clone)]
pub struct LambertMaterial {
    pub double_side: bool,
    pub texture: Option<Sampler2D>,
    pub ambient: Color,
    pub diffuse: Color,
    pub bump_texture: Option<Sampler2D>,
    pub bump_strength: f32,
    pub use_shadow: bool,
}

impl LambertMaterial {
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

impl Default for LambertMaterial {
    fn default() -> Self {
        Self {
            double_side: true,
            texture: None,
            ambient: color::black(),
            diffuse: color::white(),
            bump_texture: None,
            bump_strength: 1.0,
            use_shadow: false,
        }
    }
}

impl UpdateProgramUniforms for LambertMaterial {
    fn update_program_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {
        self.update_struct_uniform(context, "material", program);
        self.use_shadow
            .update_uniform(context, "useShadow", program);
    }
}

impl GenericMaterial for LambertMaterial {
    fn vertex_shader(&self) -> &str {
        include_str!("vertex.glsl")
    }

    fn fragment_shader(&self) -> &str {
        include_str!("fragment.glsl")
    }
}
