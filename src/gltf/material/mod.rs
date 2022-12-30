use glm::Vec4;
use web_sys::WebGl2RenderingContext;

use crate::{
    base::color,
    core::{
        material::{GenericMaterial, TextureRef},
        program::{Program, UpdateProgramUniforms, UpdateUniform},
        texture::TextureUnit,
    },
};

const USE_LIGHT: bool = true;

#[derive(Debug)]
pub struct TestMaterial {
    pub base_color_factor: Vec4,
    pub use_light: bool,
    pub min_factor: f32,
    pub base_color_texture: Option<TextureRef>,
}

impl Default for TestMaterial {
    fn default() -> Self {
        Self {
            base_color_factor: color::white(),
            use_light: USE_LIGHT,
            min_factor: 0.2,
            base_color_texture: None,
        }
    }
}

impl GenericMaterial for TestMaterial {
    fn vertex_shader(&self) -> &str {
        include_str!("test.vert")
    }

    fn fragment_shader(&self) -> &str {
        include_str!("test.frag")
    }
}

impl UpdateProgramUniforms for TestMaterial {
    fn update_program_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {
        self.base_color_factor
            .update_uniform(context, "u_BaseColorFactor", program);
        self.use_light
            .update_uniform(context, "u_UseLight", program);
        self.min_factor
            .update_uniform(context, "u_MinFactor", program);

        if let Some(base_color_texture) = &self.base_color_texture {
            let sampler = TextureUnit(0);
            sampler.active_texture(context);
            base_color_texture.texture().bind(context);
            sampler.update_uniform(context, "u_BaseColorSampler", program);
        }
        self.base_color_texture
            .is_some()
            .update_uniform(context, "u_UseTexture", program);
    }
}
