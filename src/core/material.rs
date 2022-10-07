pub mod basic;
pub mod sprite;
pub mod texture;

use std::collections::HashMap;

use anyhow::Result;
use glm::Mat4;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

use super::{
    convert::FromWithContext,
    gl,
    uniform::{Uniform, UniformData},
};

#[derive(Debug, Clone)]
pub struct Material {
    program: WebGlProgram,
    uniforms: HashMap<String, Uniform>,
    render_settings: Vec<RenderSetting>,
    pub draw_style: u32,
    model_matrix: Uniform,
    view_matrix: Uniform,
    projection_matrix: Uniform,
}

impl Material {
    pub fn add_uniform(
        &mut self,
        context: &WebGl2RenderingContext,
        name: &str,
        data: UniformData,
    ) -> Result<()> {
        self.uniforms.insert(
            String::from(name),
            Uniform::new_with_data(context, data, &self.program, name)?,
        );
        Ok(())
    }

    pub fn add_render_setting(&mut self, settings: RenderSetting) {
        self.render_settings.push(settings)
    }

    pub fn program(&self) -> &WebGlProgram {
        &self.program
    }

    pub fn set_model_matrix(&self, matrix: Mat4) {
        Self::set_matrix(&self.model_matrix, matrix);
    }

    pub fn set_view_matrix(&self, matrix: Mat4) {
        Self::set_matrix(&self.view_matrix, matrix);
    }

    pub fn set_projection_matrix(&self, matrix: Mat4) {
        Self::set_matrix(&self.projection_matrix, matrix);
    }

    pub fn upload_uniform_data(&self, context: &WebGl2RenderingContext) {
        for uniform in self.uniforms.values() {
            uniform.upload_data(context);
        }
        self.model_matrix.upload_data(context);
        self.view_matrix.upload_data(context);
        self.projection_matrix.upload_data(context);
    }

    pub fn update_render_settings(&self, context: &WebGl2RenderingContext) {
        for render_setting in self.render_settings.iter() {
            render_setting.update(context);
        }
    }

    pub fn uniform(&self, name: &str) -> Option<&Uniform> {
        self.uniforms.get(name)
    }

    fn set_matrix(uniform: &Uniform, matrix: Mat4) {
        let mut m = uniform.mat4_mut().unwrap();
        *m = matrix;
    }
}

pub struct MaterialSettings<'a> {
    pub vertex_shader: &'a str,
    pub fragment_shader: &'a str,
    pub uniforms: Vec<(&'a str, UniformData)>,
    pub render_settings: Vec<RenderSetting>,
    pub draw_style: u32,
}

impl FromWithContext<WebGl2RenderingContext, MaterialSettings<'_>> for Material {
    fn from_with_context(
        context: &WebGl2RenderingContext,
        settings: MaterialSettings<'_>,
    ) -> Result<Self> {
        let program = gl::build_program(context, settings.vertex_shader, settings.fragment_shader)?;
        let uniforms: Result<Vec<_>> = settings
            .uniforms
            .into_iter()
            .map(|(name, data)| {
                Ok((
                    String::from(name),
                    Uniform::new_with_data(context, data, &program, name)?,
                ))
            })
            .collect();
        let model_matrix = Uniform::new_with_data(
            context,
            UniformData::from(Mat4::default()),
            &program,
            "modelMatrix",
        )?;
        let view_matrix = Uniform::new_with_data(
            context,
            UniformData::from(Mat4::default()),
            &program,
            "viewMatrix",
        )?;
        let projection_matrix = Uniform::new_with_data(
            context,
            UniformData::from(Mat4::default()),
            &program,
            "projectionMatrix",
        )?;
        Ok(Material {
            program,
            uniforms: uniforms?.into_iter().collect(),
            render_settings: settings.render_settings,
            draw_style: settings.draw_style,
            model_matrix,
            view_matrix,
            projection_matrix,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RenderSetting {
    LineWidth(f32),
    CullFace(bool),
}

impl RenderSetting {
    pub fn update(self, context: &WebGl2RenderingContext) {
        match self {
            RenderSetting::LineWidth(setting) => context.line_width(setting),
            RenderSetting::CullFace(setting) => {
                Self::set_capability(context, WebGl2RenderingContext::CULL_FACE, setting)
            }
        }
    }

    fn set_capability(context: &WebGl2RenderingContext, capability: u32, enabled: bool) {
        if enabled {
            context.enable(capability)
        } else {
            context.disable(capability)
        }
    }
}
