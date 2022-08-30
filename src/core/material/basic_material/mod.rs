use anyhow::Result;
use web_sys::WebGl2RenderingContext;

use crate::core::{color::Color, convert::FromWithContext, uniform::UniformData};

use super::{Material, MaterialSettings, RenderSetting};

pub struct BasicMaterial {
    pub base_color: Color,
    pub use_vertex_colors: bool,
}

impl Default for BasicMaterial {
    fn default() -> Self {
        Self {
            base_color: Color::white(),
            use_vertex_colors: false,
        }
    }
}

fn basic_material(
    context: &WebGl2RenderingContext,
    draw_style: u32,
    basic_material: BasicMaterial,
) -> Result<Material> {
    Material::from_with_context(
        context,
        MaterialSettings {
            vertex_shader: include_str!("basic.vs"),
            fragment_shader: include_str!("basic.fs"),
            uniforms: [
                ("baseColor", UniformData::from(basic_material.base_color)),
                (
                    "useVertexColors",
                    UniformData::from(basic_material.use_vertex_colors),
                ),
            ],
            draw_style,
            model_matrix: "modelMatrix",
            view_matrix: "viewMatrix",
            projection_matrix: "projectionMatrix",
        },
    )
}

pub struct PointMaterial {
    pub point_size: f32,
    pub rounded_points: bool,
}

impl Default for PointMaterial {
    fn default() -> Self {
        Self {
            point_size: 8.0,
            rounded_points: false,
        }
    }
}

pub fn point_material(
    context: &WebGl2RenderingContext,
    basic_material: BasicMaterial,
    point_material: PointMaterial,
) -> Result<Material> {
    let mut material =
        self::basic_material(context, WebGl2RenderingContext::POINTS, basic_material)?;
    material.add_uniform(
        context,
        "pointSize",
        UniformData::from(point_material.point_size),
    )?;
    Ok(material)
}

pub enum LineType {
    Connected,
    Loop,
    Segments,
}

pub struct LineMaterial {
    pub line_width: f32,
    pub line_type: LineType,
}

impl Default for LineMaterial {
    fn default() -> Self {
        Self {
            line_width: 1.0,
            line_type: LineType::Connected,
        }
    }
}

pub fn line_material(
    context: &WebGl2RenderingContext,
    basic_material: BasicMaterial,
    line_material: LineMaterial,
) -> Result<Material> {
    let draw_style = match line_material.line_type {
        LineType::Connected => WebGl2RenderingContext::LINE_STRIP,
        LineType::Loop => WebGl2RenderingContext::LINE_LOOP,
        LineType::Segments => WebGl2RenderingContext::LINES,
    };
    let mut material = self::basic_material(context, draw_style, basic_material)?;
    material.add_render_setting(RenderSetting::LineWidth(line_material.line_width));
    Ok(material)
}
