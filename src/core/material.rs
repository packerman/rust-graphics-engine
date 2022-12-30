use std::rc::Rc;

use anyhow::Result;
use web_sys::WebGl2RenderingContext;

use crate::base::{convert::FromWithContext, util::shared_ref::SharedRef};

use super::{
    program::{Program, UpdateProgramUniforms, UpdateUniform},
    texture::Texture,
};

pub trait GenericMaterial: UpdateProgramUniforms {
    fn vertex_shader(&self) -> &str;

    fn fragment_shader(&self) -> &str;

    fn preferred_mode(&self) -> Option<u32> {
        None
    }

    fn double_sided(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone)]
pub struct Material {
    #[allow(dead_code)]
    name: Option<String>,
    double_sided: bool,
    program: Program,
    generic_material: SharedRef<dyn GenericMaterial>,
    alpha_mode: AlphaMode,
}

impl Material {
    pub fn initialize(
        context: &WebGl2RenderingContext,
        name: Option<String>,
        double_sided: bool,
        generic_material: SharedRef<dyn GenericMaterial>,
        alpha_mode: AlphaMode,
    ) -> Result<Self> {
        let program = Program::initialize(
            context,
            generic_material.borrow().vertex_shader(),
            generic_material.borrow().fragment_shader(),
        )?;
        Ok(Self {
            name,
            double_sided,
            generic_material,
            program,
            alpha_mode,
        })
    }

    pub fn update(&self, context: &WebGl2RenderingContext) {
        self.update_settings(context);
        self.alpha_mode
            .update_program_uniforms(context, self.program());
        self.generic_material
            .borrow()
            .update_program_uniforms(context, self.program());
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

    pub fn update_uniform<T>(&self, context: &WebGl2RenderingContext, name: &str, value: T)
    where
        T: UpdateUniform,
    {
        value.update_uniform(context, name, &self.program);
    }

    pub fn preferred_mode(&self) -> Option<u32> {
        self.generic_material.borrow().preferred_mode()
    }

    fn update_setting(context: &WebGl2RenderingContext, setting: u32, value: bool) {
        if value {
            context.enable(setting);
        } else {
            context.disable(setting);
        }
    }
}

impl<T> FromWithContext<WebGl2RenderingContext, SharedRef<T>> for Material
where
    T: GenericMaterial,
{
    fn from_with_context(context: &WebGl2RenderingContext, value: SharedRef<T>) -> Result<Self> {
        Self::initialize(
            context,
            None,
            value.borrow().double_sided(),
            value,
            AlphaMode::default(),
        )
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

impl UpdateProgramUniforms for AlphaMode {
    fn update_program_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {
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

#[derive(Debug, Clone)]
struct DefaultGlobalUniformUpdater;

impl UpdateProgramUniforms for DefaultGlobalUniformUpdater {
    fn update_program_uniforms(&self, _context: &WebGl2RenderingContext, _program: &Program) {}
}

pub fn default_uniform_updater() -> Box<dyn UpdateProgramUniforms> {
    Box::new(DefaultGlobalUniformUpdater)
}
