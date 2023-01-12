use anyhow::Result;
use glm::{vec2, vec3, Vec2};
use web_sys::WebGl2RenderingContext;

use crate::{
    api::geometry::{Geometry, TypedGeometry},
    base::{color, convert::FromWithContext},
};

use super::util;

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

impl TryFrom<Rectangle> for TypedGeometry {
    type Error = anyhow::Error;

    fn try_from(rectangle: Rectangle) -> Result<Self> {
        let (x, y) = (rectangle.position.x, rectangle.position.y);
        let (a, b) = (rectangle.alignment.x, rectangle.alignment.y);
        let points = [
            vec3(x + (-a) * rectangle.width, y + (-b) * rectangle.height, 0.0),
            vec3(
                x + (1.0 - a) * rectangle.width,
                y + (-b) * rectangle.height,
                0.0,
            ),
            vec3(
                x + (-a) * rectangle.width,
                y + (1.0 - b) * rectangle.height,
                0.0,
            ),
            vec3(
                x + (1.0 - a) * rectangle.width,
                y + (1.0 - b) * rectangle.height,
                0.0,
            ),
        ];
        let colors = [color::white(), color::red(), color::lime(), color::blue()];
        let uvs = [
            vec2(0.0, 0.0),
            vec2(1.0, 0.0),
            vec2(0.0, 1.0),
            vec2(1.0, 1.0),
        ];
        let indices = [0, 1, 3, 0, 3, 2];
        let normal_data = util::replicate(6, vec3(0.0, 0.0, 1.0)).collect::<Vec<_>>();
        TypedGeometry::new(
            util::select_by_indices(&points, indices),
            Some(util::select_by_indices(&uvs, indices)),
            Some(normal_data),
            Some(util::select_by_indices(&colors, indices)),
        )
    }
}

impl FromWithContext<WebGl2RenderingContext, Rectangle> for Geometry {
    fn from_with_context(context: &WebGl2RenderingContext, rectangle: Rectangle) -> Result<Self> {
        let typed_geometry = TypedGeometry::try_from(rectangle)?;
        Geometry::from_with_context(context, typed_geometry)
    }
}
