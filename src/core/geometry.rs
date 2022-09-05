pub mod parametric;

use std::{
    collections::{hash_map, HashMap},
    f32::consts::TAU,
};

use anyhow::{anyhow, Result};
use glm::Mat4;
use web_sys::WebGl2RenderingContext;

use super::{
    attribute::{Attribute, AttributeData},
    color::Color,
    convert::FromWithContext,
    matrix::Angle,
};

pub struct Geometry {
    attributes: HashMap<String, Attribute>,
}

impl Geometry {
    pub fn new(attributes: HashMap<String, Attribute>) -> Self {
        Self { attributes }
    }

    pub fn attributes(&self) -> hash_map::Iter<String, Attribute> {
        self.attributes.iter()
    }

    pub fn count_vertices(&self) -> i32 {
        self.attributes
            .values()
            .next()
            .expect("Expected at least one attribute")
            .count()
    }

    pub fn appply_matrix_mut(
        &mut self,
        context: &WebGl2RenderingContext,
        matrix: &Mat4,
        name: &str,
    ) -> Result<()> {
        let attribute = self
            .attributes
            .get_mut(name)
            .ok_or_else(|| anyhow!("Cannot find attribute {}", name))?;
        attribute.apply_matrix_mut(context, matrix);
        Ok(())
    }

    pub fn merge_mut(&mut self, context: &WebGl2RenderingContext, other: Geometry) -> Result<()> {
        for (name, attribute) in self.attributes.iter_mut() {
            attribute.concat_mut(
                context,
                other
                    .attributes
                    .get(name)
                    .ok_or_else(|| anyhow!("Cannot find attribute {:?}", name))?,
            )?;
        }
        Ok(())
    }
}

impl<const N: usize> FromWithContext<WebGl2RenderingContext, [(&str, AttributeData); N]>
    for Geometry
{
    fn from_with_context(
        context: &WebGl2RenderingContext,
        attributes: [(&str, AttributeData); N],
    ) -> Result<Self> {
        let mut map = HashMap::new();
        for (name, data) in attributes {
            map.insert(String::from(name), Attribute::new_with_data(context, data)?);
        }
        Ok(Geometry::new(map))
    }
}

struct Rectangle {
    width: f32,
    height: f32,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            width: 1.0,
            height: 1.0,
        }
    }
}

impl FromWithContext<WebGl2RenderingContext, Rectangle> for Geometry {
    fn from_with_context(context: &WebGl2RenderingContext, rectangle: Rectangle) -> Result<Self> {
        let points = [
            [-rectangle.width / 2.0, -rectangle.height / 2.0, 0.0],
            [rectangle.width / 2.0, -rectangle.height / 2.0, 0.0],
            [-rectangle.width / 2.0, rectangle.height / 2.0, 0.0],
            [rectangle.width / 2.0, rectangle.height / 2.0, 0.0],
        ];
        let colors = [
            Color::white().into(),
            Color::red().into(),
            Color::lime().into(),
            Color::blue().into(),
        ];
        let position_data = util::select_by_indices(&points, [0, 1, 3, 0, 3, 2]);
        let color_data: Vec<[f32; 3]> = util::select_by_indices(&colors, [0, 1, 3, 0, 3, 2]);
        Geometry::from_with_context(
            context,
            [
                ("vertexPosition", AttributeData::from(&position_data)),
                ("vertexColor", AttributeData::from(&color_data)),
            ],
        )
    }
}

pub struct BoxGeometry {
    width: f32,
    height: f32,
    depth: f32,
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
    fn from_with_context(
        context: &WebGl2RenderingContext,
        box_geometry: BoxGeometry,
    ) -> Result<Self> {
        let points = [
            [
                -box_geometry.width / 2.0,
                -box_geometry.height / 2.0,
                -box_geometry.depth / 2.0,
            ],
            [
                box_geometry.width / 2.0,
                -box_geometry.height / 2.0,
                -box_geometry.depth / 2.0,
            ],
            [
                -box_geometry.width / 2.0,
                box_geometry.height / 2.0,
                -box_geometry.depth / 2.0,
            ],
            [
                box_geometry.width / 2.0,
                box_geometry.height / 2.0,
                -box_geometry.depth / 2.0,
            ],
            [
                -box_geometry.width / 2.0,
                -box_geometry.height / 2.0,
                box_geometry.depth / 2.0,
            ],
            [
                box_geometry.width / 2.0,
                -box_geometry.height / 2.0,
                box_geometry.depth / 2.0,
            ],
            [
                -box_geometry.width / 2.0,
                box_geometry.height / 2.0,
                box_geometry.depth / 2.0,
            ],
            [
                box_geometry.width / 2.0,
                box_geometry.height / 2.0,
                box_geometry.depth / 2.0,
            ],
        ];
        let colors = [
            Color::light_coral().into(),
            Color::maroon().into(),
            Color::light_green().into(),
            Color::green().into(),
            Color::medium_slate_blue().into(),
            Color::navy().into(),
        ];
        let position_data = util::select_by_indices(
            &points,
            [
                5, 1, 3, 5, 3, 7, 0, 4, 6, 0, 6, 2, 6, 7, 3, 6, 3, 2, 0, 1, 5, 0, 5, 4, 4, 5, 7, 4,
                7, 6, 1, 0, 2, 1, 2, 3,
            ],
        );
        let color_data: Vec<[f32; 3]> =
            util::select_by_indices(&colors, (0..=5).flat_map(|i| util::replicate(6, i)));
        Geometry::from_with_context(
            context,
            [
                ("vertexPosition", AttributeData::from(&position_data)),
                ("vertexColor", AttributeData::from(&color_data)),
            ],
        )
    }
}

struct Polygon {
    sides: u16,
    radius: f32,
}

impl Polygon {
    fn new(sides: u16, radius: f32) -> Self {
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

        let angle = Angle::from_radians(TAU) / polygon.sides.into();
        for n in 0..polygon.sides {
            position_data.push(glm::vec4(0.0, 0.0, 0.0, 1.0));

            position_data.push(glm::vec4(
                polygon.radius * (angle * n.into()).cos(),
                polygon.radius * (angle * n.into()).sin(),
                0.0,
                1.0,
            ));

            position_data.push(glm::vec4(
                polygon.radius * (angle * (n + 1).into()).cos(),
                polygon.radius * (angle * (n + 1).into()).sin(),
                0.0,
                1.0,
            ));

            color_data.push(Color::white());
            color_data.push(Color::red());
            color_data.push(Color::blue());
        }

        Geometry::from_with_context(
            context,
            [
                ("vertexPosition", AttributeData::from(&position_data)),
                ("vertexColor", AttributeData::from(&color_data)),
            ],
        )
    }
}

mod util {
    use std::{
        iter::{self, Repeat, Take},
        ops::Index,
    };

    pub fn select_by_indices<M, K, V, I>(indexed: &M, indices: I) -> Vec<V>
    where
        M: Index<K, Output = V>,
        I: IntoIterator<Item = K>,
        V: Copy,
    {
        indices.into_iter().map(|k| indexed[k]).collect()
    }

    pub fn replicate<T>(n: usize, t: T) -> Take<Repeat<T>>
    where
        T: Clone,
    {
        iter::repeat(t).take(n)
    }
}
