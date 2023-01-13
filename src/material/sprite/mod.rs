use std::rc::Rc;

use glm::Vec2;
use web_sys::WebGl2RenderingContext;

use crate::{
    base::color::{self, Color},
    core::{
        material::{GenericMaterial, Source},
        program::{Program, UpdateProgramUniforms, UpdateUniform},
        texture::{Texture, TextureUnit},
    },
};

#[derive(Debug)]
pub struct Properties {
    pub base_color: Color,
    pub billboard: bool,
    pub tile_number: f32,
    pub tile_count: Vec2,
    pub double_side: bool,
}

impl Default for Properties {
    fn default() -> Self {
        Self {
            base_color: color::white(),
            billboard: false,
            tile_number: -1.0,
            tile_count: glm::vec2(1.0, 1.0),
            double_side: true,
        }
    }
}

impl UpdateProgramUniforms for Properties {
    fn update_program_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {
        self.base_color
            .update_uniform(context, "baseColor", program);
        self.billboard.update_uniform(context, "billboard", program);
        self.tile_number
            .update_uniform(context, "tileNumber", program);
        self.tile_count
            .update_uniform(context, "tileCount", program);
    }
}

#[derive(Debug)]
pub struct SpriteMaterial {
    pub properties: Properties,
    pub texture: Rc<Texture>,
    pub unit: TextureUnit,
}

impl SpriteMaterial {
    pub fn set_tile_number(&mut self, tile_number: f32) {
        self.properties.tile_number = tile_number;
    }
}

impl UpdateProgramUniforms for SpriteMaterial {
    fn update_program_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {
        self.properties.update_program_uniforms(context, program);
        self.unit.active_texture(context);
        self.texture.bind(context);
        self.unit.update_uniform(context, "texture0", program);
    }
}

impl GenericMaterial for SpriteMaterial {
    fn vertex_shader(&self) -> Source<'_> {
        include_str!("vertex.glsl").into()
    }

    fn fragment_shader(&self) -> Source<'_> {
        include_str!("fragment.glsl").into()
    }

    fn double_sided(&self) -> bool {
        self.properties.double_side
    }
}
