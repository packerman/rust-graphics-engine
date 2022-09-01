use std::{cell::RefCell, f32::consts::TAU, rc::Rc};

use anyhow::Result;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

use crate::core::{
    application::Application,
    camera::Camera,
    convert::FromWithContext,
    geometry::{BoxGeometry, Geometry},
    input::KeyState,
    material::basic_material::{self, BasicMaterial, SurfaceMaterial},
    matrix::Angle,
    mesh::Mesh,
    node::{Node, Transform},
    renderer::{Renderer, RendererOptions},
    web,
};

pub struct SpinningCube {
    renderer: Renderer,
    scene: Rc<Node>,
    mesh: Rc<Node>,
    camera: Rc<Node>,
}

impl SpinningCube {
    pub fn create(
        context: &WebGl2RenderingContext,
        canvas: &HtmlCanvasElement,
    ) -> Result<Box<dyn Application>> {
        log!("Initializing...");

        let renderer = Renderer::new_initialized(context, RendererOptions::default());
        let scene = Node::new_group();

        let camera = RefCell::new(Camera::default());
        let (width, height) = web::canvas_size(canvas);
        camera.borrow_mut().set_aspect_ratio(width, height);
        let camera = Node::new_with_camera(camera);
        camera.set_position(&glm::vec3(0.0, 0.0, 3.0));
        scene.add_child(&camera);

        let geometry = Geometry::from_with_context(context, BoxGeometry::default())?;
        let mut basic_material = BasicMaterial::default();
        basic_material.use_vertex_colors = true;
        let material =
            basic_material::surface_material(context, basic_material, SurfaceMaterial::default())?;
        let mesh = Mesh::new(context, geometry, material)?;
        let mesh = Node::new_with_mesh(mesh);
        scene.add_child(&mesh);

        Ok(Box::new(SpinningCube {
            renderer,
            mesh,
            scene,
            camera,
        }))
    }
}

impl Application for SpinningCube {
    fn update(&mut self, _key_state: &KeyState) {
        self.mesh
            .rotate_y(Angle::from_radians(TAU) / 150.0, Transform::Local);
        self.mesh
            .rotate_x(Angle::from_radians(TAU) / 200.0, Transform::Local);
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}
