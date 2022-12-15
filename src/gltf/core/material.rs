use std::rc::Rc;

use anyhow::Result;
use glm::Vec4;
use web_sys::WebGl2RenderingContext;

use crate::gltf::program::{Program, UpdateUniforms};

#[derive(Debug, Clone)]
pub struct Material {
    name: Option<String>,
    double_sided: bool,
    program: Program,
    uniform_updater: Rc<dyn UpdateUniforms>,
}

impl Material {
    pub fn initialize(
        context: &WebGl2RenderingContext,
        name: Option<String>,
        double_sided: bool,
        uniform_updater: Rc<dyn UpdateUniforms>,
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
