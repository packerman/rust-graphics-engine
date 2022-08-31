pub mod basic_material;

use std::collections::HashMap;

use anyhow::Result;
use glm::Mat4;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

use self::basic_material::BasicMaterial;
use super::{
    convert::FromWithContext,
    gl,
    uniform::{Uniform, UniformData},
};

pub struct Material {
    pub program: WebGlProgram,
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
}

pub struct MaterialSettings<'a, const N: usize> {
    vertex_shader: &'a str,
    fragment_shader: &'a str,
    uniforms: [(&'a str, UniformData); N],
    draw_style: u32,
    model_matrix: &'a str,
    view_matrix: &'a str,
    projection_matrix: &'a str,
}

impl<const N: usize> FromWithContext<WebGl2RenderingContext, MaterialSettings<'_, N>> for Material {
    fn from_with_context(
        context: &WebGl2RenderingContext,
        settings: MaterialSettings<'_, N>,
    ) -> Result<Self> {
        let program = gl::build_program(context, settings.vertex_shader, settings.fragment_shader)?;
        let uniforms: Result<Vec<_>> = settings
            .uniforms
            .iter()
            .copied()
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
            settings.model_matrix,
        )?;
        let view_matrix = Uniform::new_with_data(
            context,
            UniformData::from(Mat4::default()),
            &program,
            settings.view_matrix,
        )?;
        let projection_matrix = Uniform::new_with_data(
            context,
            UniformData::from(Mat4::default()),
            &program,
            settings.projection_matrix,
        )?;
        Ok(Material {
            program,
            uniforms: uniforms?.into_iter().collect(),
            render_settings: vec![],
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
