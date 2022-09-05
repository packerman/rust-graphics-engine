use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

use crate::core::{
    application::Application,
    camera::Camera,
    color::Color,
    convert::FromWithContext,
    extras::{AxesHelper, GridHelper},
    input::KeyState,
    matrix::Angle,
    mesh::Mesh,
    node::{Node, Transform},
    renderer::{Renderer, RendererOptions},
    web,
};

pub struct AxesGrid {
    renderer: Renderer,
    scene: Rc<Node>,
    camera: Rc<RefCell<Camera>>,
}

impl AxesGrid {
    pub fn create(
        context: &WebGl2RenderingContext,
        canvas: &HtmlCanvasElement,
    ) -> Result<Box<dyn Application>> {
        let renderer = Renderer::new_initialized(context, RendererOptions::default());
        let scene = Node::new_group();

        let camera = Rc::new(RefCell::new(Camera::default()));
        let (width, height) = web::canvas_size(canvas);
        camera.borrow_mut().set_aspect_ratio(width, height);
        let camera_node = Node::new_with_camera(Rc::clone(&camera));
        camera_node.set_position(&glm::vec3(0.5, 1.0, 5.0));
        scene.add_child(&camera_node);

        let axes = Box::new(Mesh::from_with_context(
            context,
            AxesHelper {
                axis_length: 2.0,
                ..Default::default()
            },
        )?);
        let axes = Node::new_with_mesh(axes);
        scene.add_child(&axes);

        let grid = Box::new(Mesh::from_with_context(
            context,
            GridHelper {
                size: 20.0,
                grid_color: Color::white(),
                center_color: Color::yellow(),
                ..Default::default()
            },
        )?);
        let grid = Node::new_with_mesh(grid);
        grid.rotate_x(-Angle::RIGHT, Transform::default());
        scene.add_child(&grid);

        Ok(Box::new(AxesGrid {
            renderer,
            scene,
            camera,
        }))
    }
}

impl Application for AxesGrid {
    fn update(&mut self, _key_state: &KeyState) {}

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}
