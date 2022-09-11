use std::{cell::RefCell, f32::consts::TAU, rc::Rc};

use anyhow::Result;
use web_sys::WebGl2RenderingContext;

use crate::core::{
    application::Application,
    camera::Camera,
    convert::FromWithContext,
    geometry::{BoxGeometry, Geometry},
    input::KeyState,
    material::{
        basic::{BasicMaterial, SurfaceMaterial},
        Material,
    },
    matrix::Angle,
    mesh::Mesh,
    node::{Node, Transform},
    renderer::{Renderer, RendererOptions},
};

pub struct SpinningCube<'a> {
    renderer: Renderer,
    scene: Rc<Node<'a>>,
    mesh: Rc<Node<'a>>,
    camera: Rc<RefCell<Camera>>,
}

impl SpinningCube<'_> {
    pub fn create(context: &WebGl2RenderingContext) -> Result<Box<dyn Application>> {
        log!("Initializing...");

        let renderer = Renderer::new_initialized(context, RendererOptions::default());
        let scene = Node::new_group();

        let camera = Rc::new(RefCell::new(Camera::default()));
        let camera_node = Node::new_camera(Rc::clone(&camera));
        camera_node.set_position(&glm::vec3(0.0, 0.0, 2.0));
        scene.add_child(&camera_node);

        let geometry = Geometry::from_with_context(context, BoxGeometry::default())?;
        let material = Material::from_with_context(
            context,
            SurfaceMaterial {
                basic: BasicMaterial {
                    use_vertex_colors: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        )?;
        let mesh = Box::new(Mesh::new(context, geometry, material)?);
        let mesh = Node::new_mesh(mesh);
        scene.add_child(&mesh);

        Ok(Box::new(SpinningCube {
            renderer,
            mesh,
            scene,
            camera,
        }))
    }
}

impl Application for SpinningCube<'_> {
    fn update(&mut self, _key_state: &KeyState) {
        self.mesh
            .rotate_y(Angle::from_radians(TAU) / 450.0, Transform::Local);
        self.mesh
            .rotate_x(Angle::from_radians(TAU) / 600.0, Transform::Local);
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}
