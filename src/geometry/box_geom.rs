use std::rc::Rc;

use anyhow::Result;
use web_sys::WebGl2RenderingContext;

use crate::{
    api::geometry::Geometry,
    base::{color, convert::FromWithContext},
    core::{accessor::Accessor, mesh},
};

use super::util;

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
                mesh::POSITION_ATTRIBUTE,
                Rc::new(Accessor::from_with_context(
                    context,
                    &util::select_by_indices(
                        &points,
                        [
                            5, 1, 3, 5, 3, 7, 0, 4, 6, 0, 6, 2, 6, 7, 3, 6, 3, 2, 0, 1, 5, 0, 5, 4,
                            4, 5, 7, 4, 7, 6, 1, 0, 2, 1, 2, 3,
                        ],
                    ),
                )?),
            ),
            (
                mesh::COLOR_0_ATTRIBUTE,
                Rc::new(Accessor::from_with_context(
                    context,
                    &util::select_by_indices(&colors, (0..=5).flat_map(|i| util::replicate(6, i))),
                )?),
            ),
            (
                mesh::TEXCOORD_0_ATTRIBUTE,
                Rc::new(Accessor::from_with_context(
                    context,
                    &util::select_by_indices(&uvs, util::cycle_n([0, 1, 3, 0, 3, 2], 6)),
                )?),
            ),
            (
                mesh::NORMAL_ATTRIBUTE,
                Rc::new(Accessor::from_with_context(context, &normal_data)?),
            ),
        ]);
        Ok(geometry)
    }
}
