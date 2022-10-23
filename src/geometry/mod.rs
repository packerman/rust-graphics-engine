pub mod parametric;
mod util;

use std::f32::consts::TAU;

use anyhow::Result;
use glm::Vec2;
use web_sys::WebGl2RenderingContext;

use crate::core::{
    attribute::AttributeData, color::Color, convert::FromWithContext, geometry::Geometry,
    math::angle::Angle,
};

pub struct Rectangle {
    pub width: f32,
    pub height: f32,
    pub position: Vec2,
    pub alignment: Vec2,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            width: 1.0,
            height: 1.0,
            position: glm::vec2(0.0, 0.0),
            alignment: glm::vec2(0.5, 0.5),
        }
    }
}

impl FromWithContext<WebGl2RenderingContext, Rectangle> for Geometry {
    fn from_with_context(context: &WebGl2RenderingContext, rectangle: Rectangle) -> Result<Self> {
        let (x, y) = (rectangle.position.x, rectangle.position.y);
        let (a, b) = (rectangle.alignment.x, rectangle.alignment.y);
        let points = [
            [x + (-a) * rectangle.width, y + (-b) * rectangle.height, 0.0],
            [
                x + (1.0 - a) * rectangle.width,
                y + (-b) * rectangle.height,
                0.0,
            ],
            [
                x + (-a) * rectangle.width,
                y + (1.0 - b) * rectangle.height,
                0.0,
            ],
            [
                x + (1.0 - a) * rectangle.width,
                y + (1.0 - b) * rectangle.height,
                0.0,
            ],
        ];
        let colors = [Color::white(), Color::red(), Color::lime(), Color::blue()];
        let uvs = [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
        let indices = [0, 1, 3, 0, 3, 2];
        let normal_data = util::replicate(6, glm::vec3(0_f32, 0.0, 0.0)).collect::<Vec<_>>();
        Self::from_with_context(
            context,
            [
                (
                    "vertexPosition",
                    AttributeData::from(&util::select_by_indices(&points, indices)),
                ),
                (
                    "vertexColor",
                    AttributeData::from(&util::select_by_indices(&colors, indices)),
                ),
                (
                    "vertexUV",
                    AttributeData::from(&util::select_by_indices(&uvs, indices)),
                ),
                ("vertexNormal", AttributeData::from(&normal_data)),
                ("faceNormal", AttributeData::from(&normal_data)),
            ],
        )
    }
}

pub struct BoxGeometry {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
}

impl Default for BoxGeometry {
    fn default() -> Self {
        Self {
            width: 1.0,
            height: 1.0,
            depth: 1.0,
        }
    }
}

impl FromWithContext<WebGl2RenderingContext, BoxGeometry> for Geometry {
    fn from_with_context(context: &WebGl2RenderingContext, value: BoxGeometry) -> Result<Self> {
        let points = [
            [-value.width / 2.0, -value.height / 2.0, -value.depth / 2.0],
            [value.width / 2.0, -value.height / 2.0, -value.depth / 2.0],
            [-value.width / 2.0, value.height / 2.0, -value.depth / 2.0],
            [value.width / 2.0, value.height / 2.0, -value.depth / 2.0],
            [-value.width / 2.0, -value.height / 2.0, value.depth / 2.0],
            [value.width / 2.0, -value.height / 2.0, value.depth / 2.0],
            [-value.width / 2.0, value.height / 2.0, value.depth / 2.0],
            [value.width / 2.0, value.height / 2.0, value.depth / 2.0],
        ];
        let colors = [
            Color::light_coral(),
            Color::maroon(),
            Color::light_green(),
            Color::green(),
            Color::medium_slate_blue(),
            Color::navy(),
        ];
        let uvs = [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
        Self::from_with_context(
            context,
            [
                (
                    "vertexPosition",
                    AttributeData::from(&util::select_by_indices(
                        &points,
                        [
                            5, 1, 3, 5, 3, 7, 0, 4, 6, 0, 6, 2, 6, 7, 3, 6, 3, 2, 0, 1, 5, 0, 5, 4,
                            4, 5, 7, 4, 7, 6, 1, 0, 2, 1, 2, 3,
                        ],
                    )),
                ),
                (
                    "vertexColor",
                    AttributeData::from(&util::select_by_indices(
                        &colors,
                        (0..=5).flat_map(|i| util::replicate(6, i)),
                    )),
                ),
                (
                    "vertexUV",
                    AttributeData::from(&util::select_by_indices(
                        &uvs,
                        util::cycle_n([0, 1, 3, 0, 3, 2], 6),
                    )),
                ),
            ],
        )
    }
}

pub struct Polygon {
    pub sides: u16,
    pub radius: f32,
}

impl Polygon {
    pub fn new(sides: u16, radius: f32) -> Self {
        Self { sides, radius }
    }
}

impl Default for Polygon {
    fn default() -> Self {
        Self {
            sides: 3,
            radius: 1.0,
        }
    }
}

impl FromWithContext<WebGl2RenderingContext, Polygon> for Geometry {
    fn from_with_context(context: &WebGl2RenderingContext, polygon: Polygon) -> Result<Self> {
        let mut position_data = Vec::with_capacity((3 * polygon.sides).into());
        let mut color_data = Vec::with_capacity((3 * polygon.sides).into());
        let mut texture_data = Vec::with_capacity((3 * polygon.sides).into());

        let angle = Angle::from_radians(TAU) / polygon.sides.into();
        for n in 0..polygon.sides {
            position_data.push(glm::vec3(0.0, 0.0, 0.0));
            position_data.push(glm::vec3(
                polygon.radius * (angle * n.into()).cos(),
                polygon.radius * (angle * n.into()).sin(),
                0.0,
            ));
            position_data.push(glm::vec3(
                polygon.radius * (angle * (n + 1).into()).cos(),
                polygon.radius * (angle * (n + 1).into()).sin(),
                0.0,
            ));

            color_data.push(Color::white());
            color_data.push(Color::red());
            color_data.push(Color::blue());

            texture_data.push(glm::vec2(0.5, 0.5));
            texture_data.push(glm::vec2(
                (angle * n.into()).cos() * 0.5 + 0.5,
                (angle * n.into()).sin() * 0.5 + 0.5,
            ));
            texture_data.push(glm::vec2(
                (angle * (n + 1).into()).cos() * 0.5 + 0.5,
                (angle * (n + 1).into()).sin() * 0.5 + 0.5,
            ));
        }

        Self::from_with_context(
            context,
            [
                ("vertexPosition", AttributeData::from(&position_data)),
                ("vertexColor", AttributeData::from(&color_data)),
                ("vertexUV", AttributeData::from(&texture_data)),
            ],
        )
    }
}
