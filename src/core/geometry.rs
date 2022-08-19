use std::{
    collections::{hash_map, HashMap},
    f32::consts::PI,
};

use anyhow::Result;
use web_sys::WebGl2RenderingContext;

use super::{attribute::Attribute, color::Color, convert::FromWithContext, matrix::Angle};

pub struct Geometry {
    attributes: HashMap<String, Attribute>,
}

impl Geometry {
    fn new() -> Self {
        Geometry {
            attributes: HashMap::new(),
        }
    }

    fn from_attributes<const N: usize>(attributes: [(&str, Attribute); N]) -> Self {
        let mut map = HashMap::new();
        for (name, attribute) in attributes {
            map.insert(String::from(name), attribute);
        }
        Geometry { attributes: map }
    }

    pub fn attributes(&self) -> hash_map::Iter<String, Attribute> {
        self.attributes.iter()
    }

    pub fn count_vertices(&self) -> usize {
        self.attributes
            .values()
            .next()
            .expect("Expected at least one attribute")
            .vertex_count
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
    fn from(context: &WebGl2RenderingContext, rectangle: Rectangle) -> Result<Self> {
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
        let color_data = util::select_by_indices(&colors, [0, 1, 3, 0, 3, 2]);
        let geometry = Geometry::from_attributes([
            (
                "vertexPosition",
                Attribute::from_array(context, &position_data)?,
            ),
            ("vertexColor", Attribute::from_array(context, &color_data)?),
        ]);
        Ok(geometry)
    }
}

struct Box {
    width: f32,
    height: f32,
    depth: f32,
}

impl Default for Box {
    fn default() -> Self {
        Self {
            width: 1.0,
            height: 1.0,
            depth: 1.0,
        }
    }
}

impl FromWithContext<WebGl2RenderingContext, Box> for Geometry {
    fn from(context: &WebGl2RenderingContext, value: Box) -> Result<Self> {
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
                7, 6, 1, 0, 2, 1, 3, 3,
            ],
        );
        let color_data =
            util::select_by_indices(&colors, (0..=5).flat_map(|i| util::replicate(6, i)));
        let geometry = Geometry::from_attributes([
            (
                "vertexPosition",
                Attribute::from_array(context, &position_data)?,
            ),
            ("vertexColor", Attribute::from_array(context, &color_data)?),
        ]);
        Ok(geometry)
    }
}

struct Polygon {
    sides: u16,
    radius: f32,
}

impl Polygon {
    fn hexagon(radius: f32) -> Self {
        Polygon { sides: 6, radius }
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
    fn from(context: &WebGl2RenderingContext, polygon: Polygon) -> Result<Self> {
        let angle = Angle::from_radians(2.0 * PI) / polygon.sides.into();
        let mut position_data = Vec::with_capacity((3 * polygon.sides).into());
        let mut color_data = Vec::with_capacity((3 * polygon.sides).into());
        for n in 0..polygon.sides {
            position_data.push([0.0, 0.0, 0.0]);
            position_data.push([
                polygon.radius * (angle * n.into()).cos(),
                polygon.radius * (angle * n.into()).sin(),
                0.0,
            ]);
            position_data.push([
                polygon.radius * (angle * (n + 1).into()).cos(),
                polygon.radius * (angle * (n + 1).into()).sin(),
                0.0,
            ]);
            color_data.push(Color::white().into());
            color_data.push(Color::red().into());
            color_data.push(Color::blue().into());
        }
        let geometry = Geometry::from_attributes([
            (
                "vertexPosition",
                Attribute::from_array(context, &position_data)?,
            ),
            ("vertexColor", Attribute::from_array(context, &color_data)?),
        ]);
        Ok(geometry)
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
