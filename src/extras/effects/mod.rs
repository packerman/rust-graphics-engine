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

fn create_basic(
    context: &WebGl2RenderingContext,
    fragment_shader: &str,
    sampler_2d: Sampler2D,
) -> Result<Effect> {
    Material::from_with_context(
        context,
        MaterialSettings {
            vertex_shader: include_str!("effect.vert"),
            fragment_shader,
            uniforms: vec![("texture0", UniformData::from(sampler_2d))],
            render_settings: vec![],
            draw_style: WebGl2RenderingContext::TRIANGLES,
        },
    )
}

pub fn tint(
    context: &WebGl2RenderingContext,
    sampler_2d: Sampler2D,
    tint_color: Color,
) -> Result<Effect> {
    let mut effect = create_basic(context, include_str!("tint.frag"), sampler_2d)?;
    effect.add_uniform(context, "tintColor", UniformData::from(tint_color))?;
    Ok(effect)
}

#[allow(dead_code)]
pub fn invert(context: &WebGl2RenderingContext, sampler_2d: Sampler2D) -> Result<Effect> {
    create_basic(context, include_str!("invert.frag"), sampler_2d)
}

pub fn pixelate(
    context: &WebGl2RenderingContext,
    sampler_2d: Sampler2D,
    pixel_size: u16,
    resolution: Vec2,
) -> Result<Effect> {
    let mut effect = create_basic(context, include_str!("pixelate.frag"), sampler_2d)?;
    effect.add_uniform(
        context,
        "pixelSize",
        UniformData::from(f32::from(pixel_size)),
    )?;
    effect.add_uniform(context, "resolution", UniformData::from(resolution))?;
    Ok(effect)
}

#[allow(dead_code)]
pub fn vignette(
    context: &WebGl2RenderingContext,
    sampler_2d: Sampler2D,
    dim_start: f32,
    dim_end: f32,
    dim_color: Color,
) -> Result<Effect> {
    let mut effect = create_basic(context, include_str!("vignette.frag"), sampler_2d)?;
    effect.add_uniform(context, "dimStart", UniformData::from(dim_start))?;
    effect.add_uniform(context, "dimEnd", UniformData::from(dim_end))?;
    effect.add_uniform(context, "dimColor", UniformData::from(dim_color))?;
    Ok(effect)
}

pub fn color_reduce(
    context: &WebGl2RenderingContext,
    sampler_2d: Sampler2D,
    levels: u16,
) -> Result<Effect> {
    let mut effect = create_basic(context, include_str!("color_reduce.frag"), sampler_2d)?;
    effect.add_uniform(context, "levels", UniformData::from(f32::from(levels)))?;
    Ok(effect)
}
