use std::rc::Rc;

use anyhow::Result;
use glm::Mat4;
use web_sys::{WebGl2RenderingContext, WebGlVertexArrayObject};

use super::{camera::Camera, geometry::Geometry, gl, material::Material};

#[derive(Debug, Clone)]
pub struct Mesh {
    vao: WebGlVertexArrayObject,
    visible: bool,
    geometry: Rc<Geometry>,
    material: Rc<Material>,
}

impl Mesh {
    pub fn initialize(
        context: &WebGl2RenderingContext,
        geometry: Rc<Geometry>,
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
        self.render_with_material(context, &self.material, camera, model_matrix)
    }

    pub fn render_with_material(
        &self,
        context: &WebGl2RenderingContext,
        material: &Material,
        camera: &Camera,
        model_matrix: Mat4,
    ) {
        if self.visible {
            context.use_program(material.program().into());
            context.bind_vertex_array(self.vao().into());
            material.set_model_matrix(model_matrix);
            material.set_view_matrix(*camera.view_matrix());
            material.set_projection_matrix(camera.projection_matrix());
            material.upload_uniform_data(context);
            material.update_render_settings(context);
            context.draw_arrays(material.draw_style, 0, self.geometry.count_vertices())
        }
    }

    pub fn geometry_mut(&mut self) -> Option<&mut Geometry> {
        Rc::get_mut(&mut self.geometry)
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    fn vao(&self) -> &WebGlVertexArrayObject {
        &self.vao
    }
}
