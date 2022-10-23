use std::collections::{hash_map, HashMap};

use anyhow::{anyhow, Result};
use glm::Mat4;
use web_sys::WebGl2RenderingContext;

use super::{
    attribute::{Attribute, AttributeData},
    convert::FromWithContext,
};

#[derive(Debug, Clone)]
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

    pub fn apply_matrix_mut(
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

    pub fn merge_mut(&mut self, context: &WebGl2RenderingContext, other: &Geometry) -> Result<()> {
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
