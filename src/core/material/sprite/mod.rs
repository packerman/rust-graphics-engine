use std::rc::Rc;

use anyhow::Result;
use glm::Vec2;
use web_sys::WebGl2RenderingContext;

use crate::core::{
    color::Color,
    convert::FromWithContext,
    texture::{Texture, TextureUnit},
    uniform::UniformData,
};

use super::{Material, MaterialSettings, RenderSetting};

pub struct SpriteMaterial {
    pub base_color: Color,
    pub billboard: bool,
    pub tile_number: f32,
    pub tile_count: Vec2,
    pub double_side: bool,
}

impl Default for SpriteMaterial {
    fn default() -> Self {
        Self {
            base_color: Color::white(),
            billboard: false,
            tile_number: -1.0,
            tile_count: glm::vec2(1.0, 1.0),
            double_side: true,
        }
    }
}

pub fn create(
    context: &WebGl2RenderingContext,
    texture: Rc<Texture>,
    unit: TextureUnit,
    sprite_material: SpriteMaterial,
) -> Result<Material> {
    Material::from_with_context(
        context,
        MaterialSettings {
            vertex_shader: include_str!("vertex.glsl"),
            fragment_shader: include_str!("fragment.glsl"),
            uniforms: vec![
                ("baseColor", UniformData::from(sprite_material.base_color)),
                ("texture0", UniformData::sampler2d(texture, unit)),
                ("billboard", UniformData::from(sprite_material.billboard)),
                ("tileNumber", UniformData::from(sprite_material.tile_number)),
                ("tileCount", UniformData::from(sprite_material.tile_count)),
            ],
            render_settings: vec![RenderSetting::CullFace(!sprite_material.double_side)],
            draw_style: WebGl2RenderingContext::TRIANGLES,
        },
    )
}
