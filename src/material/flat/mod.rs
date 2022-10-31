use std::{collections::HashMap, rc::Rc};

use anyhow::Result;
use web_sys::WebGl2RenderingContext;

use crate::core::{
    color::Color,
    convert::FromWithContext,
    material::{Material, MaterialSettings, RenderSetting},
    uniform::data::{Data, Sampler2D},
};

pub struct FlatMaterial {
    double_side: bool,
    texture: Option<Sampler2D>,
    ambient: Color,
    diffuse: Color,
}

impl Default for FlatMaterial {
    fn default() -> Self {
        Self {
            double_side: true,
            texture: None,
            ambient: Color::black(),
            diffuse: Color::white(),
        }
    }
}

pub fn create(
    context: &WebGl2RenderingContext,
    flat_material: FlatMaterial,
) -> Result<Rc<Material>> {
    let render_settings = vec![RenderSetting::CullFace(!flat_material.double_side)];
    Material::from_with_context(
        context,
        MaterialSettings {
            vertex_shader: include_str!("vertex.glsl"),
            fragment_shader: include_str!("fragment.glsl"),
            uniforms: vec![("material", self::create_material_struct(flat_material))],
            render_settings,
            draw_style: WebGl2RenderingContext::TRIANGLES,
        },
    )
    .map(Rc::new)
}

fn create_material_struct(flat_material: FlatMaterial) -> Data {
    let mut members = HashMap::from([
        ("ambient", Data::from(flat_material.ambient)),
        ("diffuse", Data::from(flat_material.diffuse)),
    ]);
    let use_texture: bool;
    if let Some(sampler) = flat_material.texture {
        use_texture = true;
        members.insert("texture0", Data::from(sampler));
    } else {
        use_texture = false;
    }
    members.insert("useTexture", Data::from(use_texture));
    Data::from(members)
}
