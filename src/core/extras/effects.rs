use anyhow::Result;
use web_sys::WebGl2RenderingContext;

use crate::core::{
    color::Color,
    convert::FromWithContext,
    material::{Material, MaterialSettings},
    uniform::{Sampler2D, UniformData},
};

use super::postprocessor::Effect;

const TEMPLATE_VERTEX_SHADER: &str = r##"#version 300 es

in vec2 vertexPosition;
in vec2 vertexUV;
out vec2 UV;

void main()
{
    gl_Position = vec4(vertexPosition, 0.0, 1.0);
}
"##;

fn create_basic(
    context: &WebGl2RenderingContext,
    fragment_shader: &str,
    sampler_2d: Sampler2D,
) -> Result<Effect> {
    Material::from_with_context(
        context,
        MaterialSettings {
            vertex_shader: TEMPLATE_VERTEX_SHADER,
            fragment_shader,
            uniforms: vec![("texture0", UniformData::from(sampler_2d))],
            render_settings: vec![],
            draw_style: WebGl2RenderingContext::TRIANGLES,
        },
    )
}

const TINT_FRAGMENT_SHADER: &str = r##"#version 300 es

precision highp float;

in vec2 UV;
uniform sampler2D texture0;
uniform vec4 tintColor;
out vec4 fragColor;

void main()
{
    vec4 color = texture(texture0, UV);
    float gray = (color.r + color.g + color.b) / 3.0;
    fragColor = gray * tintColor;
}
"##;

pub fn tint(
    context: &WebGl2RenderingContext,
    sampler_2d: Sampler2D,
    tint_color: Color,
) -> Result<Effect> {
    let mut effect = create_basic(context, TINT_FRAGMENT_SHADER, sampler_2d)?;
    effect.add_uniform(context, "tintColor", UniformData::from(tint_color))?;
    Ok(effect)
}
