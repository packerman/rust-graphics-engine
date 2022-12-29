use std::collections::HashMap;

use anyhow::{anyhow, Result};
use glm::Mat4;
use web_sys::WebGl2RenderingContext;

use crate::base::{convert::FromWithContext, math::matrix};

use super::attribute::{Attribute, AttributeData};

impl Geometry {
    pub fn new(attributes: HashMap<String, Attribute>) -> Self {
        Self { attributes }
    }

    pub fn attributes(&self) -> impl Iterator<Item = (&String, &Attribute)> {
        self.attributes.iter()
    }

    pub fn attribute_mut(&mut self, name: &str) -> Result<&mut Attribute> {
        self.attributes
            .get_mut(name)
            .ok_or_else(|| anyhow!("Cannot find attribute {}", name))
    }

    pub fn count_vertices(&self) -> i32 {
        self.attributes
            .values()
            .next()
            .expect("Expected at least one attribute")
            .count()
    }

    pub fn apply_matrix(
        &mut self,
        context: &WebGl2RenderingContext,
        matrix: &Mat4,
        name: &str,
        rotate_attrs: &[&str],
    ) -> Result<()> {
        self.attribute_mut(name)?.apply_matrix(context, matrix);
        let rotation_matrix = matrix::get_rotation_matrix(matrix);
        for rotate_attr in rotate_attrs {
            self.attribute_mut(rotate_attr)?
                .apply_matrix3(context, &rotation_matrix)
        }
        Ok(())
    }

    pub fn apply_matrix_default(
        &mut self,
        context: &WebGl2RenderingContext,
        matrix: &Mat4,
    ) -> Result<()> {
        self.apply_matrix(
            context,
            matrix,
            "vertexPosition",
            &["vertexNormal", "faceNormal"],
        )
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
