use anyhow::Result;
use web_sys::{WebGl2RenderingContext, WebGlVertexArrayObject};

use super::{geometry::Geometry, gl, material::Material};

pub struct Mesh {
    vao: WebGlVertexArrayObject,
}

impl Mesh {
    pub fn new(
        context: &WebGl2RenderingContext,
        geometry: Geometry,
        material: Material,
    ) -> Result<Self> {
        let vao = gl::create_vertex_array(context)?;
        context.bind_vertex_array(Some(&vao));
        for (name, attribute) in geometry.attributes() {
            attribute.associate_variable(context, material.program(), name)?;
        }
        context.bind_vertex_array(None);
        Ok(Mesh { vao })
    }
}
