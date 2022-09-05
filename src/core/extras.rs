use anyhow::Result;

use web_sys::WebGl2RenderingContext;

use super::{
    attribute::AttributeData,
    color::Color,
    convert::FromWithContext,
    geometry::Geometry,
    material::{
        basic_material::{BasicMaterial, LineMaterial, LineType},
        Material,
    },
    mesh::Mesh,
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
        let geometry = Geometry::from_with_context(
            context,
            [
                ("vertexPosition", AttributeData::from(&position_data)),
                ("vertexColor", AttributeData::from(&color_data)),
            ],
        )?;
        let material = Material::from_with_context(
            context,
            LineMaterial {
                basic: BasicMaterial {
                    use_vertex_colors: true,
                    ..Default::default()
                },
                line_width: axes_helper.line_width,
                line_type: LineType::Segments,
            },
        )?;
        Mesh::new(context, geometry, material)
    }
}

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
            grid_color: Color::black(),
            center_color: Color::gray(),
            line_width: 1.0,
        }
    }
}

impl FromWithContext<WebGl2RenderingContext, GridHelper> for Mesh {
    fn from_with_context(
        context: &WebGl2RenderingContext,
        grid_helper: GridHelper,
    ) -> Result<Self> {
        let delta_size = grid_helper.size / f32::from(grid_helper.divisions);
        let values: Vec<_> = (0..=grid_helper.divisions)
            .map(|n| -grid_helper.size / 2.0 + f32::from(n) * delta_size)
            .collect();
        let mut position_data = Vec::new();
        let mut color_data = Vec::new();
        for x in values.iter().copied() {
            position_data.push(glm::vec4(x, -grid_helper.size / 2.0, 0.0, 1.0));
            position_data.push(glm::vec4(x, grid_helper.size / 2.0, 0.0, 1.0));
            if x == 0.0 {
                color_data.push(grid_helper.center_color);
                color_data.push(grid_helper.center_color);
            } else {
                color_data.push(grid_helper.grid_color);
                color_data.push(grid_helper.grid_color);
            }
        }
        for y in values {
            position_data.push(glm::vec4(-grid_helper.size / 2.0, y, 0.0, 1.0));
            position_data.push(glm::vec4(grid_helper.size / 2.0, y, 0.0, 1.0));
            if y == 0.0 {
                color_data.push(grid_helper.center_color);
                color_data.push(grid_helper.center_color);
            } else {
                color_data.push(grid_helper.grid_color);
                color_data.push(grid_helper.grid_color);
            }
        }
        let geometry = Geometry::from_with_context(
            context,
            [
                ("vertexPosition", AttributeData::from(&position_data)),
                ("vertexColor", AttributeData::from(&color_data)),
            ],
        )?;
        let material = Material::from_with_context(
            context,
            LineMaterial {
                basic: BasicMaterial {
                    use_vertex_colors: true,
                    ..Default::default()
                },
                line_width: grid_helper.line_width,
                line_type: LineType::Segments,
            },
        )?;
        Mesh::new(context, geometry, material)
    }
}
