use std::{collections::HashMap, rc::Rc};

use anyhow::Result;
use web_sys::WebGl2RenderingContext;

use crate::core::{
    color::Color,
    convert::FromWithContext,
    light::Light,
    material::{Material, MaterialSettings, RenderSetting},
    uniform::{
        self,
        data::{Data, Sampler2D},
    },
};

pub struct LambertMaterial {
    pub double_side: bool,
    pub texture: Option<Sampler2D>,
    pub ambient: Color,
    pub diffuse: Color,
    pub bump_texture: Option<Sampler2D>,
    pub bump_strength: f32,
    pub use_shadow: bool,
}

impl Default for LambertMaterial {
    fn default() -> Self {
        Self {
            double_side: true,
            texture: None,
            ambient: Color::black(),
            diffuse: Color::white(),
            bump_texture: None,
            bump_strength: 1.0,
            use_shadow: false,
        }
    }
}

pub fn create(
    context: &WebGl2RenderingContext,
    flat_material: LambertMaterial,
) -> Result<Rc<Material>> {
    let render_settings = vec![RenderSetting::CullFace(!flat_material.double_side)];
    Material::from_with_context(
        context,
        MaterialSettings {
            vertex_shader: include_str!("vertex.glsl"),
            fragment_shader: include_str!("fragment.glsl"),
            uniforms: vec![
                ("material", self::create_material_struct(flat_material)),
                ("light0", uniform::default_data::<Light>()),
                ("light1", uniform::default_data::<Light>()),
                ("light2", uniform::default_data::<Light>()),
                ("light3", uniform::default_data::<Light>()),
            ],
            render_settings,
            draw_style: WebGl2RenderingContext::TRIANGLES,
        },
    )
    .map(Rc::new)
}

fn create_material_struct(material: LambertMaterial) -> Data {
    let mut members = HashMap::from([
        ("ambient", Data::from(material.ambient)),
        ("diffuse", Data::from(material.diffuse)),
    ]);

    let use_texture: bool;
    if let Some(sampler) = material.texture {
        use_texture = true;
        members.insert("texture0", Data::from(sampler));
    } else {
        use_texture = false;
    }
    members.insert("useTexture", Data::from(use_texture));

    let use_bump_texture: bool;
    if let Some(sampler) = material.bump_texture {
        use_bump_texture = true;
        members.insert("bumpTexture", Data::from(sampler));
    } else {
        use_bump_texture = false;
    }
    members.insert("useBumpTexture", Data::from(use_bump_texture));
    members.insert("bumpStrength", Data::from(material.bump_strength));

    Data::from(members)
}
