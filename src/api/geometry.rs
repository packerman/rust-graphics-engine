use std::{collections::HashMap, rc::Rc};

use anyhow::{anyhow, Result};
use glm::{Mat4, Vec2, Vec3, Vec4};
use web_sys::WebGl2RenderingContext;

use crate::{
    base::{convert::FromWithContext, util::validate},
    core::{
        accessor::Accessor,
        material::Material,
        mesh::{self, Mesh},
    },
};

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

#[derive(Debug)]
pub struct TypedGeometry {
    position: Vec<Vec3>,
    texcoord_0: Option<Vec<Vec2>>,
    normal: Option<Vec<Vec3>>,
    color_0: Option<Vec<Vec4>>,
}

impl TypedGeometry {
    pub fn new(
        position: Vec<Vec3>,
        texcoord_0: Option<Vec<Vec2>>,
        normal: Option<Vec<Vec3>>,
        color_0: Option<Vec<Vec4>>,
    ) -> Result<Self> {
        validate::not_empty(&position, || anyhow!("Position must not be empty"));
        validate::optional(&texcoord_0, |texcoord| {
            validate::assert(texcoord.len() == position.len(), || {
                anyhow!("Vector length must be equal")
            })
        })?;
        validate::optional(&normal, |normal| {
            validate::assert(normal.len() == position.len(), || {
                anyhow!("Vector length must be equal")
            })
        })?;
        validate::optional(&color_0, |color| {
            validate::assert(color.len() == position.len(), || {
                anyhow!("Vector length must be equal")
            })
        })?;
        Ok(Self {
            position,
            texcoord_0,
            normal,
            color_0,
        })
    }

    pub fn transform_mut(&self, transform: &Mat4) {
        for vertex in self.position.iter_mut() {
            let transformed = transform * glm::vec4(vertex.x, vertex.y, vertex.z, 1.0);
            *vertex = glm::vec4_to_vec3(&transformed);
        }
    }

    pub fn concat_mut(&mut self, other: &TypedGeometry) -> Result<()> {
        validate::assert(self.has_texcoord() == other.has_texcoord(), || {
            anyhow!("TypedGeometry::concat: Number of texcoord elements must be equal")
        })?;
        validate::assert(self.has_normal() == other.has_normal(), || {
            anyhow!("TypedGeometry::concat: Number of normal elements must be equal")
        })?;
        validate::assert(self.has_color() == other.has_color(), || {
            anyhow!("TypedGeometry::concat: Number of color elements must be equal")
        })?;
        self.position.extend(&other.position);
        if let Some(texcoord) = &mut self.texcoord_0 {
            texcoord.extend(&other.texcoord_0.unwrap());
        }
        if let Some(normal) = &mut self.normal {
            normal.extend(&other.normal.unwrap());
        }
        if let Some(color) = &mut self.color_0 {
            color.extend(&other.color_0.unwrap());
        }
        Ok(())
    }

    pub fn has_texcoord(&self) -> bool {
        self.texcoord_0.is_some()
    }

    pub fn has_normal(&self) -> bool {
        self.normal.is_some()
    }

    pub fn has_color(&self) -> bool {
        self.color_0.is_some()
    }
}

impl FromWithContext<WebGl2RenderingContext, TypedGeometry> for Geometry {
    fn from_with_context(context: &WebGl2RenderingContext, value: TypedGeometry) -> Result<Self> {
        let mut attributes = HashMap::new();
        attributes.insert(
            String::from(mesh::POSITION_ATTRIBUTE),
            Rc::new(Accessor::from_with_context(context, &value.position)?),
        );
        if let Some(texcoord_0) = &value.texcoord_0 {
            attributes.insert(
                String::from(mesh::TEXCOORD_0_ATTRIBUTE),
                Rc::new(Accessor::from_with_context(context, texcoord_0)?),
            );
        }
        if let Some(normal) = &value.normal {
            attributes.insert(
                String::from(mesh::NORMAL_ATTRIBUTE),
                Rc::new(Accessor::from_with_context(context, normal)?),
            );
        }
        if let Some(color_0) = &value.color_0 {
            attributes.insert(
                String::from(mesh::COLOR_0_ATTRIBUTE),
                Rc::new(Accessor::from_with_context(context, color_0)?),
            );
        }
        Ok(Geometry::new(attributes))
    }
}
