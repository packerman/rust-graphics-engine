use std::rc::Rc;

use anyhow::Result;
use web_sys::WebGl2RenderingContext;

use crate::{
    api::geometry::Geometry,
    base::{
        color::{self, Color},
        convert::FromWithContext,
        util::shared_ref,
    },
    core::{
        accessor::Accessor,
        material::Material,
        mesh::{self, Mesh},
    },
    material::basic::{BasicMaterial, LineMaterial, LineType},
};

pub struct AxesHelper {
    pub axis_length: f32,
    pub line_width: f32,
    pub axis_colors: [Color; 3],
}

impl Default for AxesHelper {
    fn default() -> Self {
        Self {
            axis_length: 1.0,
            line_width: 4.0,
            axis_colors: [color::red(), color::green(), color::blue()],
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
        let geometry = Geometry::from([
            (
                mesh::POSITION_ATTRIBUTE,
                Rc::new(Accessor::from_with_context(context, &position_data)?),
            ),
            (
                mesh::COLOR_0_ATTRIBUTE,
                Rc::new(Accessor::from_with_context(context, &color_data)?),
            ),
        ]);
        let material = Rc::new(Material::from_with_context(
            context,
            shared_ref::strong(LineMaterial {
                basic: BasicMaterial {
                    use_vertex_colors: true,
                    ..Default::default()
                },
                line_width: axes_helper.line_width,
                line_type: LineType::Segments,
            }),
        )?);
        geometry.create_mesh(context, material)
    }
}
