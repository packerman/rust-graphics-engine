use std::{collections::HashMap, rc::Rc};

use anyhow::Result;
use web_sys::WebGl2RenderingContext;

use crate::core::{accessor::Accessor, material::Material, mesh::Mesh};

#[derive(Debug, Clone)]
pub struct Geometry {
    attributes: HashMap<String, Rc<Accessor>>,
}

impl Geometry {
    pub fn new(attributes: HashMap<String, Rc<Accessor>>) -> Self {
        Self { attributes }
    }

    pub fn create_mesh(
        &self,
        context: &WebGl2RenderingContext,
        material: Rc<Material>,
    ) -> Result<Mesh> {
        self.create_mesh_with_mode(context, material, WebGl2RenderingContext::TRIANGLES)
    }

    pub fn create_mesh_with_mode(
        &self,
        context: &WebGl2RenderingContext,
        material: Rc<Material>,
        mode: u32,
    ) -> Result<Mesh> {
        Mesh::primitive(context, self.attributes, None, material, mode)
    }
}

impl<const N: usize> From<[(&str, Rc<Accessor>); N]> for Geometry {
    fn from(accessors: [(&str, Rc<Accessor>); N]) -> Self {
        let mut map = HashMap::new();
        for (name, accessor) in accessors {
            map.insert(String::from(name), accessor);
        }
        Geometry::new(map)
    }
}
