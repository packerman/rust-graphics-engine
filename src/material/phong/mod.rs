use std::{collections::HashMap, rc::Rc};

use anyhow::Result;
use web_sys::WebGl2RenderingContext;

use crate::core::{
    color::Color,
    convert::FromWithContext,
    light::{shadow::Shadow, Light},
    material::{Material, MaterialSettings, RenderSetting},
    uniform::data::{CreateDataFromType, CreateDataFromValue, Data, Sampler2D},
};

pub struct PhongMaterial<'a> {
    pub double_side: bool,
    pub texture: Option<Sampler2D>,
    pub ambient: Color,
    pub diffuse: Color,
    pub specular_strength: f32,
    pub shininess: f32,
    pub bump_texture: Option<Sampler2D>,
    pub bump_strength: f32,
    pub shadow: Option<&'a Shadow>,
}

impl PhongMaterial<'_> {
    fn texture(&self) -> Option<Sampler2D> {
        self.texture.clone()
    }

    fn bump_texture(&self) -> Option<Sampler2D> {
        self.bump_texture.clone()
    }
}

impl Default for PhongMaterial<'_> {
    fn default() -> Self {
        Self {
            double_side: true,
            texture: None,
            ambient: Color::black(),
            diffuse: Color::white(),
            specular_strength: 1.0,
            shininess: 32.0,
            bump_texture: None,
            bump_strength: 1.0,
            shadow: None,
        }
    }
}

pub fn create(
    context: &WebGl2RenderingContext,
    phong_material: PhongMaterial,
) -> Result<Rc<Material>> {
    let render_settings = vec![RenderSetting::CullFace(!phong_material.double_side)];
    let mut material = Material::from_with_context(
        context,
        MaterialSettings {
            vertex_shader: include_str!("vertex.glsl"),
            fragment_shader: include_str!("fragment.glsl"),
            uniforms: vec![
                ("material", self::create_material_struct(&phong_material)),
                ("viewPosition", Data::from(glm::vec3(0.0, 0.0, 0.0))),
                ("light0", Light::create_data()),
                ("light1", Light::create_data()),
                ("light2", Light::create_data()),
                ("light3", Light::create_data()),
            ],
            render_settings,
            draw_style: WebGl2RenderingContext::TRIANGLES,
        },
    )?;
    if let Some(shadow) = phong_material.shadow {
        material.add_uniform(context, "useShadow", true);
        material.add_uniform(context, "shadow0", shadow.create_data());
    } else {
        material.add_uniform(context, "useShadow", false);
    }
    Ok(Rc::new(material))
}

fn create_material_struct(material: &PhongMaterial) -> Data {
    let mut members = HashMap::from([
        ("ambient", Data::from(material.ambient)),
        ("diffuse", Data::from(material.diffuse)),
        ("specularStrength", Data::from(material.specular_strength)),
        ("shininess", Data::from(material.shininess)),
    ]);
    let use_texture: bool;
    if let Some(sampler) = material.texture() {
        use_texture = true;
        members.insert("texture0", Data::from(sampler));
    } else {
        use_texture = false;
    }
    members.insert("useTexture", Data::from(use_texture));

    let use_bump_texture: bool;
    if let Some(sampler) = material.bump_texture() {
        use_bump_texture = true;
        members.insert("bumpTexture", Data::from(sampler));
    } else {
        use_bump_texture = false;
    }
    members.insert("useBumpTexture", Data::from(use_bump_texture));
    members.insert("bumpStrength", Data::from(material.bump_strength));

    Data::from(members)
}
