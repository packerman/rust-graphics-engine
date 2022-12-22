use glm::Vec4;
use web_sys::WebGl2RenderingContext;

use crate::base::color;

use super::{
    core::{
        material::{MaterialLifecycle, TextureRef},
        texture_data::TextureUnit,
    },
    program::{Program, UpdateUniform, UpdateUniforms},
};

#[derive(Debug)]
pub struct TestMaterial {
    pub base_color_factor: Vec4,
    pub min_factor: f32,
    pub base_color_texture: Option<TextureRef>,
}

impl Default for TestMaterial {
    fn default() -> Self {
        Self {
            base_color_factor: color::white(),
            min_factor: 0.2,
            base_color_texture: None,
        }
    }
}

impl MaterialLifecycle for TestMaterial {
    fn vertex_shader(&self) -> &str {
        include_str!("test.vert")
    }

    fn fragment_shader(&self) -> &str {
        include_str!("test.frag")
    }
}

impl UpdateUniforms for TestMaterial {
    fn update_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {
        self.base_color_factor
            .update_uniform(context, "u_BaseColorFactor", program);
        self.min_factor
            .update_uniform(context, "u_MinFactor", program);

        if let Some(base_color_texture) = &self.base_color_texture {
            let sampler = TextureUnit(0);
            sampler.active_texture(context);
            base_color_texture.texture().bind(context);
            sampler.update_uniform(context, "u_Sampler", program);
        }
        self.base_color_texture
            .is_some()
            .update_uniform(context, "u_UseTexture", program);
    }
}
