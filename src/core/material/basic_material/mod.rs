use std::collections::HashMap;

use anyhow::Result;
use web_sys::WebGl2RenderingContext;

use crate::core::{
    color::Color,
    gl, matrix,
    uniform::{Uniform, UniformData},
};

use super::{Material, UpdateRenderSettings};

pub struct BasicMaterial {
    settings: BasicMaterialSettings,
    material_type: BasicMaterialType,
}

pub struct BasicMaterialSettings {
    base_color: Color,
    use_vertex_colors: bool,
}

impl UpdateRenderSettings for BasicMaterial {
    fn update_render_settings(&self, context: &WebGl2RenderingContext) {
        todo!()
    }
}

pub enum BasicMaterialType {
    Point {
        point_size: f32,
        rounded_points: bool,
    },
}

impl UpdateRenderSettings for BasicMaterialType {
    fn update_render_settings(&self, context: &WebGl2RenderingContext) {
        todo!()
    }
}

pub fn basic_material(
    context: &WebGl2RenderingContext,
    basic_material_type: BasicMaterialType,
) -> Result<Material> {
    let vertex_shader_source = include_str!("basic.vs");
    let fragment_shader_source = include_str!("basic.fs");
    let program = gl::build_program(context, vertex_shader_source, fragment_shader_source)?;
    Ok(Material {
        uniforms: HashMap::from([
            (
                String::from("baseColor"),
                Uniform::new_with_data(
                    context,
                    UniformData::from(Color::white()),
                    &program,
                    "baseColor",
                )?,
            ),
            (
                String::from("useVertexColors"),
                Uniform::new_with_data(
                    context,
                    UniformData::from(false),
                    &program,
                    "useVertexColors",
                )?,
            ),
        ]),
        draw_style: WebGl2RenderingContext::POINTS,
        model_matrix: Uniform::new_with_data(
            context,
            UniformData::from(matrix::identity()),
            &program,
            "modelMatrix",
        )?,
        view_matrix: Uniform::new_with_data(
            context,
            UniformData::from(matrix::identity()),
            &program,
            "viewMatrix",
        )?,
        projection_matrix: Uniform::new_with_data(
            context,
            UniformData::from(matrix::identity()),
            &program,
            "projectionMatrix",
        )?,
        material_type: super::MaterialType::BasicMaterial(BasicMaterial {
            settings: BasicMaterialSettings {
                base_color: Color::white(),
                use_vertex_colors: false,
            },
            material_type: basic_material_type,
        }),
        program,
    })
}

pub fn point_material(context: &WebGl2RenderingContext) -> Result<Material> {
    basic_material(
        context,
        BasicMaterialType::Point {
            point_size: 8.0,
            rounded_points: false,
        },
    )
}
