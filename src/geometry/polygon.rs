use std::f32::consts::TAU;

use anyhow::Result;
use web_sys::WebGl2RenderingContext;

use crate::{
    api::geometry::{Geometry, TypedGeometry},
    base::{color, convert::FromWithContext, math::angle::Angle},
};

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

impl TryFrom<Polygon> for TypedGeometry {
    type Error = anyhow::Error;

    fn try_from(polygon: Polygon) -> Result<Self> {
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

        TypedGeometry::new(
            position_data,
            Some(texture_data),
            Some(normal_data),
            Some(color_data),
        )
    }
}

impl FromWithContext<WebGl2RenderingContext, Polygon> for Geometry {
    fn from_with_context(context: &WebGl2RenderingContext, polygon: Polygon) -> Result<Self> {
        let typed_geometry = TypedGeometry::try_from(polygon)?;
        Geometry::from_with_context(context, typed_geometry)
    }
}
