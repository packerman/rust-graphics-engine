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

pub struct TextureMaterial {
    pub base_color: Color,
    pub repeat_uv: Vec2,
    pub offset_uv: Vec2,
    pub double_side: bool,
}

impl Default for TextureMaterial {
    fn default() -> Self {
        Self {
            base_color: Color::white(),
            repeat_uv: glm::vec2(1.0, 1.0),
            offset_uv: glm::vec2(0.0, 0.0),
            double_side: true,
        }
    }
}

pub fn create(
    context: &WebGl2RenderingContext,
    texture: Rc<Texture>,
    unit: TextureUnit,
    texture_material: TextureMaterial,
) -> Result<Rc<Material>> {
    Material::from_with_context(
        context,
        MaterialSettings {
            vertex_shader: include_str!("vertex.glsl"),
            fragment_shader: include_str!("fragment.glsl"),
            uniforms: vec![
                ("baseColor", UniformData::from(texture_material.base_color)),
                ("textureSampler", UniformData::sampler2d(texture, unit)),
                ("repeatUV", UniformData::from(texture_material.repeat_uv)),
                ("offsetUV", UniformData::from(texture_material.offset_uv)),
            ],
            render_settings: vec![RenderSetting::CullFace(!texture_material.double_side)],
            draw_style: WebGl2RenderingContext::TRIANGLES,
        },
    )
    .map(Rc::new)
}
