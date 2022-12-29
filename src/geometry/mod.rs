pub mod parametric;
mod util;

use std::f32::consts::TAU;

use anyhow::Result;
use glm::Vec2;
use web_sys::WebGl2RenderingContext;

use crate::{
    api::geometry::Geometry,
    base::{color, convert::FromWithContext, math::angle::Angle},
    core::{accessor::Accessor, mesh::Primitive},
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
        let colors = [color::white(), color::red(), color::lime(), color::blue()];
        let uvs = [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
        let indices = [0, 1, 3, 0, 3, 2];
        let normal_data = util::replicate(6, [0.0, 0.0, 1.0]).collect::<Vec<_>>();
        let geometry = Self::from([
            (
                Primitive::POSITION_ATTRIBUTE,
                Accessor::from_with_context(context, &util::select_by_indices(&points, indices))?,
            ),
            (
                Primitive::COLOR_0_ATTRIBUTE,
                Accessor::from_with_context(context, &util::select_by_indices(&colors, indices))?,
            ),
            (
                Primitive::TEXCOORD_0_ATTRIBUTE,
                Accessor::from_with_context(context, &util::select_by_indices(&uvs, indices))?,
            ),
            (
                Primitive::NORMAL_ATTRIBUTE,
                Accessor::from_with_context(context, &normal_data),
            )?,
        ]);
        Ok(geometry)
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
            color::light_coral(),
            color::maroon(),
            color::light_green(),
            color::green(),
            color::medium_slate_blue(),
            color::navy(),
        ];
        let uvs = [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
        let normals = [
            [1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, -1.0],
        ];
        let normal_data = util::select_by_indices(&normals, util::replicate_each(6, 0..6));
        let geometry = Self::from([
            (
                Primitive::POSITION_ATTRIBUTE,
                Accessor::from_with_context(
                    context,
                    &util::select_by_indices(
                        &points,
                        [
                            5, 1, 3, 5, 3, 7, 0, 4, 6, 0, 6, 2, 6, 7, 3, 6, 3, 2, 0, 1, 5, 0, 5, 4,
                            4, 5, 7, 4, 7, 6, 1, 0, 2, 1, 2, 3,
                        ],
                    ),
                ),
            )?,
            (
                Primitive::COLOR_0_ATTRIBUTE,
                Accessor::from_with_context(
                    context,
                    &util::select_by_indices(&colors, (0..=5).flat_map(|i| util::replicate(6, i))),
                )?,
            ),
            (
                Primitive::TEXCOORD_0_ATTRIBUTE,
                Accessor::from_with_context(
                    context,
                    &util::select_by_indices(&uvs, util::cycle_n([0, 1, 3, 0, 3, 2], 6)),
                )?,
            ),
            (
                Primitive::NORMAL_ATTRIBUTE,
                Accessor::from_with_context(context, &normal_data)?,
            ),
        ]);
        Ok(geometry)
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
        let mut normal_data = Vec::with_capacity((3 * polygon.sides).into());
        let normal_vector = glm::vec3(0.0, 0.0, 1.0);

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

            color_data.push(color::white());
            color_data.push(color::red());
            color_data.push(color::blue());

            texture_data.push(glm::vec2(0.5, 0.5));
            texture_data.push(glm::vec2(
                (angle * n.into()).cos() * 0.5 + 0.5,
                (angle * n.into()).sin() * 0.5 + 0.5,
            ));
            texture_data.push(glm::vec2(
                (angle * (n + 1).into()).cos() * 0.5 + 0.5,
                (angle * (n + 1).into()).sin() * 0.5 + 0.5,
            ));

            normal_data.push(normal_vector);
            normal_data.push(normal_vector);
            normal_data.push(normal_vector);
        }

        let geometry = Self::from([
            (
                Primitive::POSITION_ATTRIBUTE,
                Accessor::from_with_context(context, &position_data)?,
            ),
            (
                Primitive::COLOR_0_ATTRIBUTE,
                Accessor::from_with_context(context, &color_data)?,
            ),
            (
                Primitive::TEXCOORD_0_ATTRIBUTE,
                Accessor::from_with_context(context, &texture_data)?,
            ),
            (
                Primitive::NORMAL_ATTRIBUTE,
                Accessor::from_with_context(context, &normal_data)?,
            ),
        ]);
        Ok(geometry)
    }
}
