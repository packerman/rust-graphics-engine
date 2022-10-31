use std::collections::HashMap;

use anyhow::{anyhow, Result};
use glm::Mat4;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

use super::{
    convert::FromWithContext,
    gl,
    uniform::{data::Data, Uniform},
};

#[derive(Debug, Clone)]
pub struct Material {
    program: WebGlProgram,
    uniforms: HashMap<String, Uniform>,
    render_settings: Vec<RenderSetting>,
    pub draw_style: u32,
    model_matrix: Option<Uniform>,
    view_matrix: Option<Uniform>,
    projection_matrix: Option<Uniform>,
}

impl Material {
    pub fn add_uniform(&mut self, context: &WebGl2RenderingContext, name: &str, data: Data) {
        if let Some(uniform) = Uniform::from_data(context, &self.program, name, data) {
            self.uniforms.insert(String::from(name), uniform);
        }
    }

    pub fn add_render_setting(&mut self, settings: RenderSetting) {
        self.render_settings.push(settings)
    }

    pub fn program(&self) -> &WebGlProgram {
        &self.program
    }

    pub fn set_model_matrix(&self, matrix: Mat4) {
        Self::set_matrix_uniform(self.model_matrix.as_ref(), matrix);
    }

    pub fn set_view_matrix(&self, matrix: Mat4) {
        Self::set_matrix_uniform(self.view_matrix.as_ref(), matrix);
    }

    pub fn set_projection_matrix(&self, matrix: Mat4) {
        Self::set_matrix_uniform(self.projection_matrix.as_ref(), matrix);
    }

    pub fn upload_uniform_data(&self, context: &WebGl2RenderingContext) {
        for uniform in self.uniforms.values() {
            uniform.upload_data(context);
        }
        if let Some(uniform) = &self.model_matrix {
            uniform.upload_data(context);
        }
        if let Some(uniform) = &self.view_matrix {
            uniform.upload_data(context);
        }
        if let Some(uniform) = &self.projection_matrix {
            uniform.upload_data(context);
        }
    }

    pub fn update_render_settings(&self, context: &WebGl2RenderingContext) {
        for render_setting in self.render_settings.iter() {
            render_setting.update(context);
        }
    }

    pub fn uniform(&self, name: &str) -> Option<&Uniform> {
        self.uniforms.get(name)
    }

    fn set_matrix_uniform(uniform: Option<&Uniform>, matrix: Mat4) {
        if let Some(uniform) = uniform {
            let mut m = uniform.mat4_mut().unwrap();
            *m = matrix;
        }
    }
}

pub struct MaterialSettings<'a> {
    pub vertex_shader: &'a str,
    pub fragment_shader: &'a str,
    pub uniforms: Vec<(&'a str, Data)>,
    pub render_settings: Vec<RenderSetting>,
    pub draw_style: u32,
}

impl FromWithContext<WebGl2RenderingContext, MaterialSettings<'_>> for Material {
    fn from_with_context(
        context: &WebGl2RenderingContext,
        settings: MaterialSettings<'_>,
    ) -> Result<Self> {
        self::check_draw_style_is_correct(settings.draw_style)?;
        let program = gl::build_program(context, settings.vertex_shader, settings.fragment_shader)?;
        let uniforms: HashMap<_, _> = settings
            .uniforms
            .into_iter()
            .filter_map(|(name, data)| {
                Some((
                    String::from(name),
                    Uniform::from_data(context, &program, name, data)?,
                ))
            })
            .collect();
        let model_matrix = Uniform::from_default::<Mat4>(context, &program, "modelMatrix");
        let view_matrix = Uniform::from_default::<Mat4>(context, &program, "viewMatrix");
        let projection_matrix =
            Uniform::from_default::<Mat4>(context, &program, "projectionMatrix");
        Ok(Material {
            program,
            uniforms,
            render_settings: settings.render_settings,
            draw_style: settings.draw_style,
            model_matrix,
            view_matrix,
            projection_matrix,
        })
    }
}

const DRAW_STYLES: [u32; 7] = [
    WebGl2RenderingContext::POINTS,
    WebGl2RenderingContext::LINES,
    WebGl2RenderingContext::LINE_LOOP,
    WebGl2RenderingContext::LINE_STRIP,
    WebGl2RenderingContext::TRIANGLES,
    WebGl2RenderingContext::TRIANGLE_STRIP,
    WebGl2RenderingContext::TRIANGLE_FAN,
];

fn check_draw_style_is_correct(draw_style: u32) -> Result<()> {
    if DRAW_STYLES.contains(&draw_style) {
        Ok(())
    } else {
        Err(anyhow!("Unkown draw style: {:#?}", draw_style))
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
