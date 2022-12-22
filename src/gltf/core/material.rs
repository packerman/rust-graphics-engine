use std::rc::Rc;

use anyhow::Result;
use web_sys::WebGl2RenderingContext;

use crate::gltf::program::{Program, UpdateUniforms};

use super::texture_data::Texture;

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
}

impl Material {
    pub fn initialize(
        context: &WebGl2RenderingContext,
        name: Option<String>,
        double_sided: bool,
        uniform_updater: Rc<dyn MaterialLifecycle>,
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
        })
    }

    pub fn update(&self, context: &WebGl2RenderingContext) {
        self.uniform_updater
            .update_uniforms(context, self.program());
        self.update_settings(context);
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
