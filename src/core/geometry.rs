use std::collections::{hash_map, HashMap};

use anyhow::Result;
use web_sys::WebGl2RenderingContext;

use super::{attribute::Attribute, color::Color};

pub struct Geometry {
    attributes: HashMap<String, Attribute>,
}

impl<'a> Geometry {
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

trait ToGeometry<'a> {
    fn to_geometry(&self, context: &WebGl2RenderingContext) -> Result<Geometry>;
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

impl<'a> ToGeometry<'a> for Rectangle {
    fn to_geometry(&self, context: &WebGl2RenderingContext) -> Result<Geometry> {
        let p0 = [-self.width / 2.0, -self.height / 2.0, 0.0];
        let p1 = [self.width / 2.0, -self.height / 2.0, 0.0];
        let p2 = [-self.width / 2.0, self.height / 2.0, 0.0];
        let p3 = [self.width / 2.0, self.height / 2.0, 0.0];
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
