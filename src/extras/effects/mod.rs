use glm::Vec2;
use web_sys::WebGl2RenderingContext;

use crate::{
    base::{color::Color, math::resolution::Resolution},
    core::{
        material::{GenericMaterial, Source},
        program::{Program, UpdateProgramUniforms, UpdateUniform},
    },
    legacy::texture::Sampler2D,
};

#[derive(Debug, Clone)]
struct BaseEffect {
    texture_0: Sampler2D,
}

impl BaseEffect {
    pub const VERTEX_SHADER: &str = include_str!("effect.vert");

    fn new(texture_0: Sampler2D) -> Self {
        Self { texture_0 }
    }
}

impl UpdateProgramUniforms for BaseEffect {
    fn update_program_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {
        self.texture_0.update_uniform(context, "texture0", program);
    }
}

#[derive(Debug)]
pub struct TintEffect {
    base: BaseEffect,
    tint_color: Color,
}

impl UpdateProgramUniforms for TintEffect {
    fn update_program_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {
        self.update_program_uniforms(context, program);
        self.tint_color
            .update_uniform(context, "tintColor", program);
    }
}

impl GenericMaterial for TintEffect {
    fn vertex_shader(&self) -> Source<'_> {
        BaseEffect::VERTEX_SHADER.into()
    }

    fn fragment_shader(&self) -> Source<'_> {
        include_str!("tint.frag").into()
    }
}

pub fn tint(
    context: &WebGl2RenderingContext,
    sampler_2d: Sampler2D,
    tint_color: Color,
) -> TintEffect {
    TintEffect {
        base: BaseEffect::new(sampler_2d),
        tint_color,
    }
}

#[derive(Debug)]
pub struct PixelateEffect {
    base: BaseEffect,
    pixel_size: u16,
    resolution: Resolution,
}

impl UpdateProgramUniforms for PixelateEffect {
    fn update_program_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {
        self.update_program_uniforms(context, program);
        f32::from(self.pixel_size).update_uniform(context, "pixelSize", program);
        Vec2::from(self.resolution).update_uniform(context, "resolution", program);
    }
}

impl GenericMaterial for PixelateEffect {
    fn vertex_shader(&self) -> Source<'_> {
        BaseEffect::VERTEX_SHADER.into()
    }

    fn fragment_shader(&self) -> Source<'_> {
        include_str!("pixelate.frag").into()
    }
}

pub fn pixelate(
    context: &WebGl2RenderingContext,
    sampler_2d: Sampler2D,
    pixel_size: u16,
    resolution: Resolution,
) -> PixelateEffect {
    PixelateEffect {
        base: BaseEffect::new(sampler_2d),
        pixel_size,
        resolution,
    }
}

#[derive(Debug)]
pub struct ColorReduceEffect {
    base: BaseEffect,
    levels: u16,
}

impl UpdateProgramUniforms for ColorReduceEffect {
    fn update_program_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {
        self.base.update_program_uniforms(context, program);
        f32::from(self.levels).update_uniform(context, "levels", program);
    }
}

impl GenericMaterial for ColorReduceEffect {
    fn vertex_shader(&self) -> Source<'_> {
        BaseEffect::VERTEX_SHADER.into()
    }

    fn fragment_shader(&self) -> Source<'_> {
        include_str!("color_reduce.frag").into()
    }
}

pub fn color_reduce(
    context: &WebGl2RenderingContext,
    sampler_2d: Sampler2D,
    levels: u16,
) -> ColorReduceEffect {
    ColorReduceEffect {
        base: BaseEffect::new(sampler_2d),
        levels,
    }
}

#[derive(Debug)]
pub struct BrightFilterEffect {
    base: BaseEffect,
    filter: BrightFilter,
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

impl UpdateProgramUniforms for BrightFilter {
    fn update_program_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {
        self.threshold.update_uniform(context, "threshold", program);
    }
}

impl UpdateProgramUniforms for BrightFilterEffect {
    fn update_program_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {
        self.base.update_program_uniforms(context, program);
        self.filter.update_program_uniforms(context, program);
    }
}

impl GenericMaterial for BrightFilterEffect {
    fn vertex_shader(&self) -> Source<'_> {
        BaseEffect::VERTEX_SHADER.into()
    }

    fn fragment_shader(&self) -> Source<'_> {
        include_str!("bright_filter.frag").into()
    }
}

pub fn bright_filter(
    context: &WebGl2RenderingContext,
    sampler_2d: Sampler2D,
    bright_filter: BrightFilter,
) -> BrightFilterEffect {
    BrightFilterEffect {
        base: BaseEffect::new(sampler_2d),
        filter: bright_filter,
    }
}

#[derive(Debug)]
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

impl UpdateProgramUniforms for Blur {
    fn update_program_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {
        Vec2::from(self.texture_size).update_uniform(context, "textureSize", program);
        self.blur_radius
            .update_uniform(context, "blurRadius", program);
    }
}

#[derive(Debug)]
pub struct HorizontalBlurEffect {
    base: BaseEffect,
    blur: Blur,
}

impl UpdateProgramUniforms for HorizontalBlurEffect {
    fn update_program_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {
        self.base.update_program_uniforms(context, program);
        self.blur.update_program_uniforms(context, program);
    }
}

impl GenericMaterial for HorizontalBlurEffect {
    fn vertex_shader(&self) -> Source<'_> {
        BaseEffect::VERTEX_SHADER.into()
    }

    fn fragment_shader(&self) -> Source<'_> {
        include_str!("horizontal_blur.frag").into()
    }
}

pub fn horizontal_blur(
    context: &WebGl2RenderingContext,
    sampler_2d: Sampler2D,
    blur: Blur,
) -> HorizontalBlurEffect {
    HorizontalBlurEffect {
        base: BaseEffect::new(sampler_2d),
        blur,
    }
}

#[derive(Debug)]
pub struct VerticalBlurEffect {
    base: BaseEffect,
    blur: Blur,
}

impl UpdateProgramUniforms for VerticalBlurEffect {
    fn update_program_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {
        self.base.update_program_uniforms(context, program);
        self.blur.update_program_uniforms(context, program);
    }
}

impl GenericMaterial for VerticalBlurEffect {
    fn vertex_shader(&self) -> Source<'_> {
        BaseEffect::VERTEX_SHADER.into()
    }

    fn fragment_shader(&self) -> Source<'_> {
        include_str!("vertical_blur.frag").into()
    }
}

pub fn vertical_blur(
    context: &WebGl2RenderingContext,
    sampler_2d: Sampler2D,
    blur: Blur,
) -> VerticalBlurEffect {
    VerticalBlurEffect {
        base: BaseEffect::new(sampler_2d),
        blur,
    }
}

#[derive(Debug)]
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

impl UpdateProgramUniforms for Blend {
    fn update_program_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {
        self.original_strength
            .update_uniform(context, "originalStrength", program);
        self.blend_strength
            .update_uniform(context, "blendStrength", program);
    }
}

#[derive(Debug)]
pub struct BlendEffect {
    base: BaseEffect,
    blend: Blend,
    blend_texture: Sampler2D,
}

impl UpdateProgramUniforms for BlendEffect {
    fn update_program_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {
        self.base.update_program_uniforms(context, program);
        self.blend.update_program_uniforms(context, program);
        self.blend_texture
            .update_uniform(context, "blendTexture", program);
    }
}

impl GenericMaterial for BlendEffect {
    fn vertex_shader(&self) -> Source<'_> {
        BaseEffect::VERTEX_SHADER.into()
    }

    fn fragment_shader(&self) -> Source<'_> {
        include_str!("additive_blend.frag").into()
    }
}

pub fn additive_blend(
    context: &WebGl2RenderingContext,
    original_texture: Sampler2D,
    blend_texture: Sampler2D,
    blend: Blend,
) -> BlendEffect {
    BlendEffect {
        base: BaseEffect::new(original_texture),
        blend,
        blend_texture,
    }
}
