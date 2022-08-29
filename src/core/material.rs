pub mod basic_material;

use std::collections::HashMap;

use web_sys::{WebGl2RenderingContext, WebGlProgram};

use self::basic_material::BasicMaterial;
use super::uniform::Uniform;

pub struct Material {
    program: WebGlProgram,
    uniforms: HashMap<String, Uniform>,
    draw_style: u32,
    model_matrix: Uniform,
    view_matrix: Uniform,
    projection_matrix: Uniform,
    material_type: MaterialType,
}

impl Material {
    pub fn program(&self) -> &WebGlProgram {
        &self.program
    }
}

pub trait UpdateRenderSettings {
    fn update_render_settings(&self, context: &WebGl2RenderingContext);
}

impl UpdateRenderSettings for Material {
    fn update_render_settings(&self, context: &WebGl2RenderingContext) {
        todo!()
    }
}

pub enum MaterialType {
    BasicMaterial(BasicMaterial),
}
