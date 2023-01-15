use std::rc::Rc;

use anyhow::Result;
use web_sys::WebGl2RenderingContext;

use crate::{
    api::geometry::{Geometry, TypedGeometry},
    base::{
        color::{self, Color},
        convert::FromWithContext,
        util::shared_ref,
    },
    core::{material::Material, mesh::Mesh},
    material::basic::{BasicMaterial, LineMaterial, LineType},
};

#[derive(Debug, Clone, Copy)]
pub struct GridHelper {
    pub size: f32,
    pub divisions: u16,
    pub grid_color: Color,
    pub center_color: Color,
    pub line_width: f32,
}

impl Default for GridHelper {
    fn default() -> Self {
        Self {
            size: 10.0,
            divisions: 10,
            grid_color: color::black(),
            center_color: color::gray(),
            line_width: 1.0,
        }
    }
}

impl TryFrom<GridHelper> for TypedGeometry {
    type Error = anyhow::Error;

    fn try_from(grid_helper: GridHelper) -> Result<Self, Self::Error> {
        let delta_size = grid_helper.size / f32::from(grid_helper.divisions);
        let values: Vec<_> = (0..=grid_helper.divisions)
            .map(|n| -grid_helper.size / 2.0 + f32::from(n) * delta_size)
            .collect();
        let mut position_data = Vec::new();
        let mut color_data = Vec::new();
        for x in values.iter().copied() {
            position_data.push(glm::vec3(x, -grid_helper.size / 2.0, 0.0));
            position_data.push(glm::vec3(x, grid_helper.size / 2.0, 0.0));
            if x == 0.0 {
                color_data.push(grid_helper.center_color);
                color_data.push(grid_helper.center_color);
            } else {
                color_data.push(grid_helper.grid_color);
                color_data.push(grid_helper.grid_color);
            }
        }
        for y in values {
            position_data.push(glm::vec3(-grid_helper.size / 2.0, y, 0.0));
            position_data.push(glm::vec3(grid_helper.size / 2.0, y, 0.0));
            if y == 0.0 {
                color_data.push(grid_helper.center_color);
                color_data.push(grid_helper.center_color);
            } else {
                color_data.push(grid_helper.grid_color);
                color_data.push(grid_helper.grid_color);
            }
        }
        TypedGeometry::new(position_data, None, None, Some(color_data))
    }
}

impl FromWithContext<WebGl2RenderingContext, GridHelper> for Rc<Material> {
    fn from_with_context(
        context: &WebGl2RenderingContext,
        grid_helper: GridHelper,
    ) -> Result<Self> {
        <Rc<Material>>::from_with_context(
            context,
            shared_ref::new(LineMaterial {
                basic: BasicMaterial {
                    use_vertex_colors: true,
                    ..Default::default()
                },
                line_width: grid_helper.line_width,
                line_type: LineType::Segments,
            }),
        )
    }
}

impl FromWithContext<WebGl2RenderingContext, GridHelper> for Rc<Mesh> {
    fn from_with_context(
        context: &WebGl2RenderingContext,
        grid_helper: GridHelper,
    ) -> Result<Self> {
        let typed_geometry = TypedGeometry::try_from(grid_helper)?;
        let geometry = Geometry::from_with_context(context, typed_geometry)?;
        let material = <Rc<Material>>::from_with_context(context, grid_helper)?;
        Mesh::initialize(context, &geometry, material)
    }
}
