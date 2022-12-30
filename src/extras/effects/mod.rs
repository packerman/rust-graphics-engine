use std::rc::Rc;

use anyhow::Result;
use glm::Vec2;
use web_sys::WebGl2RenderingContext;

use crate::{
    base::{color::Color, math::resolution::Resolution},
    core::{
        material::{Material, ProgramCreator},
        program::{Program, UpdateProgramUniforms},
    },
    legacy::texture::Sampler2D,
};

use super::postprocessor::Effect;

fn create_basic(
    context: &WebGl2RenderingContext,
    fragment_shader: &str,
    sampler_2d: Sampler2D,
) -> Result<Effect> {
    Material::from_with_context(
        context,
        Rc::new(BaseEffect {
            texture_0: sampler_2d,
            fragment_shader,
        }), // TODO
            // MaterialSettings {
            //     vertex_shader: include_str!("effect.vert"),
            //     fragment_shader,
            //     uniforms: vec![("texture0", Data::from(sampler_2d))],
            //     render_settings: vec![],
            //     draw_style: WebGl2RenderingContext::TRIANGLES,
            // },
    )
}

#[derive(Debug, Clone)]
struct BaseEffect<'a> {
    texture_0: Sampler2D,
    fragment_shader: &'a str,
}

impl ProgramCreator for BaseEffect<'_> {
    fn vertex_shader(&self) -> &str {
        include_str!("effect.vert")
    }

    fn fragment_shader(&self) -> &str {
        self.fragment_shader
    }
}

impl UpdateProgramUniforms for BaseEffect<'_> {
    fn update_program_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {}
}

pub fn tint(
    context: &WebGl2RenderingContext,
    sampler_2d: Sampler2D,
    tint_color: Color,
) -> Result<Effect> {
    let mut effect = create_basic(context, include_str!("tint.frag"), sampler_2d)?;
    effect.add_uniform(context, "tintColor", tint_color);
    Ok(effect)
}

pub fn pixelate(
    context: &WebGl2RenderingContext,
    sampler_2d: Sampler2D,
    pixel_size: u16,
    resolution: Resolution,
) -> Result<Effect> {
    let mut effect = create_basic(context, include_str!("pixelate.frag"), sampler_2d)?;
    effect.add_uniform(context, "pixelSize", f32::from(pixel_size));
    effect.add_uniform(context, "resolution", Vec2::from(resolution));
    Ok(effect)
}

pub fn color_reduce(
    context: &WebGl2RenderingContext,
    sampler_2d: Sampler2D,
    levels: u16,
) -> Result<Effect> {
    let mut effect = create_basic(context, include_str!("color_reduce.frag"), sampler_2d)?;
    effect.add_uniform(context, "levels", f32::from(levels));
    Ok(effect)
}

#[derive(Debug, Clone, Copy)]
pub struct BrightFilter {
    pub threshold: f32,
}

impl Default for BrightFilter {
    fn default() -> Self {
        Self { threshold: 2.4 }
    }
}

pub fn bright_filter(
    context: &WebGl2RenderingContext,
    sampler_2d: Sampler2D,
    bright_filter: BrightFilter,
) -> Result<Effect> {
    let mut effect = create_basic(context, include_str!("bright_filter.frag"), sampler_2d)?;
    effect.add_uniform(context, "threshold", bright_filter.threshold);
    Ok(effect)
}

pub struct Blur {
    pub texture_size: Resolution,
    pub blur_radius: i32,
}

impl Default for Blur {
    fn default() -> Self {
        Self {
            texture_size: Resolution::new(512, 512),
            blur_radius: 20,
        }
    }
}

pub fn horizontal_blur(
    context: &WebGl2RenderingContext,
    sampler_2d: Sampler2D,
    blur: Blur,
) -> Result<Effect> {
    let mut effect = create_basic(context, include_str!("horizontal_blur.frag"), sampler_2d)?;
    effect.add_uniform(context, "textureSize", Vec2::from(blur.texture_size));
    effect.add_uniform(context, "blurRadius", blur.blur_radius);
    Ok(effect)
}

pub fn vertical_blur(
    context: &WebGl2RenderingContext,
    sampler_2d: Sampler2D,
    blur: Blur,
) -> Result<Effect> {
    let mut effect = create_basic(context, include_str!("vertical_blur.frag"), sampler_2d)?;
    effect.add_uniform(context, "textureSize", Vec2::from(blur.texture_size));
    effect.add_uniform(context, "blurRadius", blur.blur_radius);
    Ok(effect)
}

pub struct Blend {
    pub original_strength: f32,
    pub blend_strength: f32,
}

impl Default for Blend {
    fn default() -> Self {
        Self {
            original_strength: 1.0,
            blend_strength: 1.0,
        }
    }
}

pub fn additive_blend(
    context: &WebGl2RenderingContext,
    original_texture: Sampler2D,
    blend_texture: Sampler2D,
    blend: Blend,
) -> Result<Effect> {
    let mut effect = create_basic(
        context,
        include_str!("additive_blend.frag"),
        original_texture,
    )?;
    effect.add_uniform(context, "blendTexture", blend_texture);
    effect.add_uniform(context, "originalStrength", blend.original_strength);
    effect.add_uniform(context, "blendStrength", blend.blend_strength);
    Ok(effect)
}
