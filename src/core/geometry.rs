use std::collections::{hash_map, HashMap};

use anyhow::Result;
use web_sys::WebGl2RenderingContext;

use super::{
    attribute::{Attribute, AttributeData},
    color,
};

pub struct Geometry {
    attributes: HashMap<String, Attribute>,
}

impl Geometry {
    fn new() -> Self {
        Geometry {
            attributes: HashMap::new(),
        }
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

    fn add_attribute<D>(
        &mut self,
        context: &WebGl2RenderingContext,
        name: &str,
        data: D,
    ) -> Result<()>
    where
        D: AttributeData,
    {
        self.attributes
            .insert(String::from(name), Attribute::new_with_data(context, data)?);
        Ok(())
    }
}

trait ToGeometry {
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

impl ToGeometry for Rectangle {
    fn to_geometry(&self, context: &WebGl2RenderingContext) -> Result<Geometry> {
        let p0 = [-self.width / 2.0, -self.height / 2.0, 0.0];
        let p1 = [self.width / 2.0, -self.height / 2.0, 0.0];
        let p2 = [-self.width / 2.0, self.height / 2.0, 0.0];
        let p3 = [self.width / 2.0, self.height / 2.0, 0.0];
        let c0 = color::to_array3(&color::white());
        let c1 = color::to_array3(&color::red());
        let c2 = color::to_array3(&color::lime());
        let c3 = color::to_array3(&color::blue());
        let position_data = [p0, p1, p3, p0, p3, p2];
        let color_data = [c0, c1, c3, c0, c3, c2];
        let mut geometry = Geometry::new();
        geometry.add_attribute(context, "vertexPosition", &position_data)?;
        geometry.add_attribute(context, "vertexColor", &color_data)?;
        Ok(geometry)
    }
}
