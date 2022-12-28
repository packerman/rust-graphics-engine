use std::rc::Rc;

use anyhow::Result;
use glm::Vec2;
use web_sys::WebGl2RenderingContext;

use crate::{
    base::{
        color::{self, Color},
        convert::FromWithContext,
    },
    core::texture::TextureUnit,
    legacy::{
        material::{Material, MaterialSettings, RenderSetting},
        texture::Texture,
        uniform::data::{Data, Sampler2D},
    },
};

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
            base_color: color::white(),
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
) -> Result<Rc<Material>> {
    Material::from_with_context(
        context,
        MaterialSettings {
            vertex_shader: include_str!("vertex.glsl"),
            fragment_shader: include_str!("fragment.glsl"),
            uniforms: vec![
                ("baseColor", Data::from(sprite_material.base_color)),
                ("texture0", Data::from(Sampler2D::new(texture, unit))),
                ("billboard", Data::from(sprite_material.billboard)),
                ("tileNumber", Data::from(sprite_material.tile_number)),
                ("tileCount", Data::from(sprite_material.tile_count)),
            ],
            render_settings: vec![RenderSetting::CullFace(!sprite_material.double_side)],
            draw_style: WebGl2RenderingContext::TRIANGLES,
        },
    )
    .map(Rc::new)
}
