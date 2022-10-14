use anyhow::Result;
use glm::Vec2;
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
    UV = vertexUV;
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
    fragColor = vec4(gray * tintColor.rgb, 1.0);
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

const INVERT_FRAGMENT_SHADER: &str = r##"#version 300 es

precision highp float;

in vec2 UV;
uniform sampler2D texture0;
uniform vec4 tintColor;
out vec4 fragColor;

void main()
{
    vec4 color = texture(texture0, UV);
    fragColor = vec4(1.0 - color.r, 1.0 - color.g, 1.0 - color.b, 1.0);
}
"##;

#[allow(dead_code)]
pub fn invert(context: &WebGl2RenderingContext, sampler_2d: Sampler2D) -> Result<Effect> {
    create_basic(context, INVERT_FRAGMENT_SHADER, sampler_2d)
}

const PIXELATE_FRAGMENT_SHADER: &str = r##"#version 300 es

precision highp float;

in vec2 UV;
uniform sampler2D texture0;
uniform float pixelSize;
uniform vec2 resolution;
out vec4 fragColor;

void main()
{
    vec2 factor = resolution / pixelSize;
    vec2 newUV = floor(UV * factor) / factor;
    fragColor = texture(texture0, newUV);
}
"##;

pub fn pixelate(
    context: &WebGl2RenderingContext,
    sampler_2d: Sampler2D,
    pixel_size: u16,
    resolution: Vec2,
) -> Result<Effect> {
    let mut effect = create_basic(context, PIXELATE_FRAGMENT_SHADER, sampler_2d)?;
    effect.add_uniform(
        context,
        "pixelSize",
        UniformData::from(f32::from(pixel_size)),
    )?;
    effect.add_uniform(context, "resolution", UniformData::from(resolution))?;
    Ok(effect)
}

const VIGNETTE_FRAGMENT_SHADER: &str = r##"#version 300 es

precision highp float;

in vec2 UV;
uniform sampler2D texture0;
uniform float dimStart;
uniform float dimEnd;
uniform vec4 dimColor;
out vec4 fragColor;

void main()
{
    vec4 color = texture(texture0, UV);

    vec2 position = 2 * UV - vec2(1.0, 1.0);
    float d = length(position);
    float b = (d - dimEnd) / (dimStart - dimEnd);
    b = clamp(b, 0.0, 1.0);

    fragColor = b * color + (1.0 - b) * dimColor;
}
"##;

#[allow(dead_code)]
pub fn vignette(
    context: &WebGl2RenderingContext,
    sampler_2d: Sampler2D,
    dim_start: f32,
    dim_end: f32,
    dim_color: Color,
) -> Result<Effect> {
    let mut effect = create_basic(context, VIGNETTE_FRAGMENT_SHADER, sampler_2d)?;
    effect.add_uniform(context, "dimStart", UniformData::from(dim_start))?;
    effect.add_uniform(context, "dimEnd", UniformData::from(dim_end))?;
    effect.add_uniform(context, "dimColor", UniformData::from(dim_color))?;
    Ok(effect)
}

const COLOR_REDUCE_FRAGMENT_SHADER: &str = r##"#version 300 es

precision highp float;

in vec2 UV;
uniform sampler2D texture0;
uniform float levels;
out vec4 fragColor;

void main()
{
    vec4 color = texture(texture0, UV);

    vec4 reduced = round(color * levels) / levels;
    reduced.a = 1.0;

    fragColor = reduced;
}
"##;

pub fn color_reduce(
    context: &WebGl2RenderingContext,
    sampler_2d: Sampler2D,
    levels: u16,
) -> Result<Effect> {
    let mut effect = create_basic(context, COLOR_REDUCE_FRAGMENT_SHADER, sampler_2d)?;
    effect.add_uniform(context, "levels", UniformData::from(f32::from(levels)))?;
    Ok(effect)
}
