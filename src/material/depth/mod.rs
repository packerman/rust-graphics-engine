use std::rc::Rc;

use anyhow::Result;
use web_sys::WebGl2RenderingContext;

use crate::{
    base::convert::FromWithContext,
    legacy::material::{Material, MaterialSettings},
};

pub fn create(context: &WebGl2RenderingContext) -> Result<Rc<Material>> {
    Material::from_with_context(
        context,
        MaterialSettings {
            vertex_shader: include_str!("vertex.glsl"),
            fragment_shader: include_str!("fragment.glsl"),
            uniforms: vec![],
            render_settings: vec![],
            draw_style: WebGl2RenderingContext::TRIANGLES,
        },
    )
    .map(Rc::new)
}
