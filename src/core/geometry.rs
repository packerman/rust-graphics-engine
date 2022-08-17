use std::{
    collections::{hash_map, HashMap},
    iter::{self, Repeat, Take},
    ops::Index,
};

use anyhow::Result;
use web_sys::WebGl2RenderingContext;

use super::{attribute::Attribute, color::Color, convert::FromWithContext};

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
        let p0 = [-rectangle.width / 2.0, -rectangle.height / 2.0, 0.0];
        let p1 = [rectangle.width / 2.0, -rectangle.height / 2.0, 0.0];
        let p2 = [-rectangle.width / 2.0, rectangle.height / 2.0, 0.0];
        let p3 = [rectangle.width / 2.0, rectangle.height / 2.0, 0.0];
        let c0 = Color::white().into();
        let c1 = Color::red().into();
        let c2 = Color::lime().into();
        let c3 = Color::blue().into();
        let position_data = [p0, p1, p3, p0, p3, p2];
        let color_data = [c0, c1, c3, c0, c3, c2];
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
        let position_data = select_by_indices(
            &points,
            [
                5, 1, 3, 5, 3, 7, 0, 4, 6, 0, 6, 2, 6, 7, 3, 6, 3, 2, 0, 1, 5, 0, 5, 4, 4, 5, 7, 4,
                7, 6, 1, 0, 2, 1, 3, 3,
            ],
        );
        let color_data = select_by_indices(&colors, (0..=5).flat_map(|i| replicate(6, i)));
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

fn select_by_indices<M, K, V, I>(indexed: &M, indices: I) -> Vec<V>
where
    M: Index<K, Output = V>,
    I: IntoIterator<Item = K>,
    V: Copy,
{
    indices.into_iter().map(|k| indexed[k]).collect()
}

fn replicate<T>(n: usize, t: T) -> Take<Repeat<T>>
where
    T: Clone,
{
    iter::repeat(t).take(n)
}
