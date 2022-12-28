use std::rc::Rc;

use anyhow::Result;
use web_sys::WebGl2RenderingContext;

use crate::gltf::program::{Program, UpdateUniform, UpdateUniforms};

use super::texture::Texture;

pub trait MaterialLifecycle: UpdateUniforms {
    fn vertex_shader(&self) -> &str;

    fn fragment_shader(&self) -> &str;
}

#[derive(Debug, Clone)]
pub struct Material {
    #[allow(dead_code)]
    name: Option<String>,
    double_sided: bool,
    program: Program,
    uniform_updater: Rc<dyn MaterialLifecycle>,
    alpha_mode: AlphaMode,
}

impl Material {
    pub fn initialize(
        context: &WebGl2RenderingContext,
        name: Option<String>,
        double_sided: bool,
        uniform_updater: Rc<dyn MaterialLifecycle>,
        alpha_mode: AlphaMode,
    ) -> Result<Self> {
        let program = Program::initialize(
            context,
            uniform_updater.vertex_shader(),
            uniform_updater.fragment_shader(),
        )?;
        Ok(Self {
            name,
            double_sided,
            uniform_updater,
            program,
            alpha_mode,
        })
    }

    pub fn update(&self, context: &WebGl2RenderingContext) {
        self.update_settings(context);
        self.alpha_mode.update_uniforms(context, self.program());
        self.uniform_updater
            .update_uniforms(context, self.program());
    }

    pub fn update_settings(&self, context: &WebGl2RenderingContext) {
        Self::update_setting(
            context,
            WebGl2RenderingContext::CULL_FACE,
            !self.double_sided,
        );
    }

    pub fn program(&self) -> &Program {
        &self.program
    }

    fn update_setting(context: &WebGl2RenderingContext, setting: u32, value: bool) {
        if value {
            context.enable(setting);
        } else {
            context.disable(setting);
        }
    }
}

#[derive(Debug, Clone)]
pub struct TextureRef {
    texture: Rc<Texture>,
    tex_coord: u32,
}

impl TextureRef {
    pub fn new(texture: Rc<Texture>, tex_coord: u32) -> Self {
        Self { texture, tex_coord }
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }
}

#[derive(Debug, Clone)]
pub enum AlphaMode {
    Opaque,
    Mask { cutoff: f32 },
    Blend,
}

impl AlphaMode {
    const OPAQUE_VALUE: i32 = 0;
    const MASK_VALUE: i32 = 1;
    const BLEND_VALUE: i32 = 2;
}

impl Default for AlphaMode {
    fn default() -> Self {
        Self::Opaque
    }
}

impl UpdateUniforms for AlphaMode {
    fn update_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {
        match self {
            Self::Opaque => {
                context.disable(WebGl2RenderingContext::BLEND);
                Self::OPAQUE_VALUE.update_uniform(context, "u_AlphaMode", program);
            }
            Self::Mask { cutoff } => {
                context.disable(WebGl2RenderingContext::BLEND);
                Self::MASK_VALUE.update_uniform(context, "u_AlphaMode", program);
                cutoff.update_uniform(context, "u_AlphaCutoff", program);
            }
            Self::Blend => {
                context.enable(WebGl2RenderingContext::BLEND);
                context.blend_func_separate(
                    WebGl2RenderingContext::SRC_ALPHA,
                    WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA,
                    WebGl2RenderingContext::ONE,
                    WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA,
                );
                context.blend_equation(WebGl2RenderingContext::FUNC_ADD);
                Self::BLEND_VALUE.update_uniform(context, "u_AlphaMode", program);
            }
        }
    }
}
