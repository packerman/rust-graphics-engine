use std::rc::Rc;

use glm::Vec2;
use web_sys::WebGl2RenderingContext;

use crate::{
    base::color::{self, Color},
    core::{
        material::GenericMaterial,
        program::{Program, UpdateProgramUniforms, UpdateUniform},
        texture::{Texture, TextureUnit},
    },
    legacy::texture::Sampler2D,
};

#[derive(Debug)]
pub struct TextureMaterial {
    properties: Properties,
    sampler: Sampler2D,
}

impl TextureMaterial {
    pub fn new(texture: Rc<Texture>, texture_unit: TextureUnit, properties: Properties) -> Self {
        Self {
            properties,
            sampler: Sampler2D::new(texture, texture_unit),
        }
    }
}

impl UpdateProgramUniforms for TextureMaterial {
    fn update_program_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {
        self.properties.update_program_uniforms(context, program);
        self.sampler
            .update_uniform(context, "textureSampler", program)
    }
}

impl GenericMaterial for TextureMaterial {
    fn vertex_shader(&self) -> &str {
        include_str!("vertex.glsl")
    }

    fn fragment_shader(&self) -> &str {
        include_str!("fragment.glsl")
    }

    fn double_sided(&self) -> bool {
        self.properties.double_side
    }
}

#[derive(Debug)]
pub struct Properties {
    pub base_color: Color,
    pub repeat_uv: Vec2,
    pub offset_uv: Vec2,
    pub double_side: bool,
}

impl Default for Properties {
    fn default() -> Self {
        Self {
            base_color: color::white(),
            repeat_uv: glm::vec2(1.0, 1.0),
            offset_uv: glm::vec2(0.0, 0.0),
            double_side: true,
        }
    }
}

impl UpdateProgramUniforms for Properties {
    fn update_program_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {
        self.base_color
            .update_uniform(context, "baseColor", program);
        self.repeat_uv.update_uniform(context, "repeatUV", program);
        self.offset_uv.update_uniform(context, "offsetUV", program)
    }
}
