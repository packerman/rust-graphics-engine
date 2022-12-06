use anyhow::Result;
use web_sys::WebGl2RenderingContext;

use crate::core::{
    convert::FromWithContext,
    material::{Material, MaterialSettings},
};

pub fn basic(context: &WebGl2RenderingContext) -> Result<Material> {
    Material::from_with_context(
        context,
        MaterialSettings {
            vertex_shader: include_str!("basic.vert"),
            fragment_shader: include_str!("basic.frag"),
            uniforms: vec![],
            render_settings: vec![],
            draw_style: WebGl2RenderingContext::TRIANGLES,
        },
    )
}
