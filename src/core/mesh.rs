use std::rc::Rc;

use anyhow::Result;
use glm::Mat4;
use web_sys::{WebGl2RenderingContext, WebGlVertexArrayObject};

use super::{camera::Camera, geometry::Geometry, gl, material::Material};

pub struct Mesh {
    vao: WebGlVertexArrayObject,
    visible: bool,
    geometry: Geometry,
    material: Rc<Material>,
}

impl Mesh {
    pub fn new(
        context: &WebGl2RenderingContext,
        geometry: Geometry,
        material: Rc<Material>,
    ) -> Result<Self> {
        let vao = gl::create_vertex_array(context)?;
        context.bind_vertex_array(Some(&vao));
        for (name, attribute) in geometry.attributes() {
            attribute.associate_variable(context, material.program(), name);
        }
        context.bind_vertex_array(None);
        Ok(Mesh {
            vao,
            visible: true,
            geometry,
            material,
        })
    }

    pub fn render(&self, context: &WebGl2RenderingContext, camera: &Camera, model_matrix: Mat4) {
        if self.visible {
            context.use_program(Some(self.material.program()));
            context.bind_vertex_array(Some(self.vao()));
            self.material.set_model_matrix(model_matrix);
            self.material.set_view_matrix(*camera.view_matrix());
            self.material
                .set_projection_matrix(camera.projection_matrix());
            self.material.upload_uniform_data(context);
            self.material.update_render_settings(context);
            context.draw_arrays(self.material.draw_style, 0, self.geometry.count_vertices())
        }
    }

    fn vao(&self) -> &WebGlVertexArrayObject {
        &self.vao
    }
}
