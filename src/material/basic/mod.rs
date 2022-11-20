use anyhow::Result;
use web_sys::WebGl2RenderingContext;

use crate::core::{
    color::Color,
    convert::FromWithContext,
    material::{Material, MaterialSettings, RenderSetting},
    uniform::data::Data,
};

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
            vertex_shader: include_str!("vertex.glsl"),
            fragment_shader: include_str!("fragment.glsl"),
            uniforms: vec![
                ("baseColor", Data::from(basic_material.base_color)),
                (
                    "useVertexColors",
                    Data::from(basic_material.use_vertex_colors),
                ),
            ],
            render_settings: vec![],
            draw_style,
        },
    )
}

pub struct PointMaterial {
    pub basic: BasicMaterial,
    pub point_size: f32,
    pub rounded_points: bool,
}

impl Default for PointMaterial {
    fn default() -> Self {
        Self {
            basic: Default::default(),
            point_size: 8.0,
            rounded_points: false,
        }
    }
}

impl FromWithContext<WebGl2RenderingContext, PointMaterial> for Material {
    fn from_with_context(
        context: &WebGl2RenderingContext,
        point_material: PointMaterial,
    ) -> Result<Self> {
        let mut material = self::basic_material(
            context,
            WebGl2RenderingContext::POINTS,
            point_material.basic,
        )?;
        material.add_uniform(context, "pointSize", point_material.point_size);
        Ok(material)
    }
}

pub enum LineType {
    Connected,
    Loop,
    Segments,
}

pub struct LineMaterial {
    pub basic: BasicMaterial,
    pub line_width: f32,
    pub line_type: LineType,
}

impl Default for LineMaterial {
    fn default() -> Self {
        Self {
            basic: Default::default(),
            line_width: 1.0,
            line_type: LineType::Connected,
        }
    }
}

impl FromWithContext<WebGl2RenderingContext, LineMaterial> for Material {
    fn from_with_context(
        context: &WebGl2RenderingContext,
        line_material: LineMaterial,
    ) -> Result<Self> {
        let draw_style = match line_material.line_type {
            LineType::Connected => WebGl2RenderingContext::LINE_STRIP,
            LineType::Loop => WebGl2RenderingContext::LINE_LOOP,
            LineType::Segments => WebGl2RenderingContext::LINES,
        };
        let mut material = self::basic_material(context, draw_style, line_material.basic)?;
        material.add_render_setting(RenderSetting::LineWidth(line_material.line_width));
        Ok(material)
    }
}

#[derive(Default)]
pub struct SurfaceMaterial {
    pub basic: BasicMaterial,
    pub double_side: bool,
}

impl FromWithContext<WebGl2RenderingContext, SurfaceMaterial> for Material {
    fn from_with_context(
        context: &WebGl2RenderingContext,
        surface_material: SurfaceMaterial,
    ) -> Result<Self> {
        let mut material = self::basic_material(
            context,
            WebGl2RenderingContext::TRIANGLES,
            surface_material.basic,
        )?;
        material.add_render_setting(RenderSetting::CullFace(!surface_material.double_side));
        Ok(material)
    }
}
