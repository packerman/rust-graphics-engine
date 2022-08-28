use std::collections::HashMap;

use glm::Vec3;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

use super::{color::Color, uniform::Uniform};

pub struct Material {
    program: WebGlProgram,
    uniforms: HashMap<String, Uniform>,
    draw_style: u32,
    model_matrix: Uniform,
    view_matrix: Uniform,
    projection_matrix: Uniform,
}

impl Material {
    pub fn program(&self) -> &WebGlProgram {
        &self.program
    }
}

struct BasicMaterial {
    base_color: Vec3,
    use_vertex_colors: bool,
    point_size: f32,
    rounded_points: bool,
}

impl Default for BasicMaterial {
    fn default() -> Self {
        Self {
            base_color: Color::white().into(),
            use_vertex_colors: false,
            point_size: 8.0,
            rounded_points: false,
        }
    }
}

struct MaterialFactory<'a> {
    context: &'a WebGl2RenderingContext,
}

impl<'a> MaterialFactory<'a> {
    fn new(context: &'a WebGl2RenderingContext) -> Self {
        Self { context }
    }
}
