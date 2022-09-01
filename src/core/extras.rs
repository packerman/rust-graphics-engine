use anyhow::Result;
use web_sys::WebGl2RenderingContext;

use super::{
    attribute::AttributeData,
    color::Color,
    convert::FromWithContext,
    geometry::Geometry,
    material::basic_material::{self, BasicMaterial, LineMaterial, LineType},
    mesh::Mesh,
};

pub struct AxesHelper {
    axis_length: f32,
    line_width: f32,
    axis_colors: [Color; 3],
}

impl Default for AxesHelper {
    fn default() -> Self {
        Self {
            axis_length: 1.0,
            line_width: 4.0,
            axis_colors: [Color::red(), Color::green(), Color::blue()],
        }
    }
}

impl FromWithContext<WebGl2RenderingContext, AxesHelper> for Mesh {
    fn from_with_context(
        context: &WebGl2RenderingContext,
        axes_helper: AxesHelper,
    ) -> Result<Self> {
        let position_data = [
            [0.0, 0.0, 0.0],
            [axes_helper.axis_length, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, axes_helper.axis_length, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 0.0, axes_helper.axis_length],
        ];
        let color_data = [
            axes_helper.axis_colors[0],
            axes_helper.axis_colors[0],
            axes_helper.axis_colors[1],
            axes_helper.axis_colors[1],
            axes_helper.axis_colors[2],
            axes_helper.axis_colors[2],
        ];
        let geometry = Geometry::from_attributes(
            context,
            [
                ("vertexPosition", AttributeData::from(&position_data)),
                ("vertexColor", AttributeData::from(&color_data)),
            ],
        )?;
        let material = basic_material::line_material(
            context,
            BasicMaterial {
                use_vertex_colors: true,
                ..Default::default()
            },
            LineMaterial {
                line_width: axes_helper.line_width,
                line_type: LineType::Segments,
            },
        )?;
        Mesh::new(context, geometry, material)
    }
}
