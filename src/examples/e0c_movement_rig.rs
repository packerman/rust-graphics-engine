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

pub struct MovementRigExample {
    renderer: Renderer,
    scene: Rc<Node>,
    camera: Rc<RefCell<Camera>>,
    rig: Rc<Node>,
}

impl MovementRigExample {
    pub fn create(
        context: &WebGl2RenderingContext,
        canvas: &HtmlCanvasElement,
    ) -> Result<Box<dyn Application>> {
        let renderer = Renderer::new_initialized(context, RendererOptions::default());
        let scene = Node::new_group();

        let camera = Rc::new(RefCell::new(Camera::default()));
        let (width, height) = web::canvas_size(canvas);
        camera.borrow_mut().set_aspect_ratio(width, height);
        let camera_node = Node::new_camera(Rc::clone(&camera));

        let rig = Node::new_movement_rig(Default::default());
        rig.add_child(&camera_node);
        rig.set_position(&glm::vec3(0.5, 1.0, 5.0));
        scene.add_child(&rig);

        let axes = Box::new(Mesh::from_with_context(
            context,
            AxesHelper {
                axis_length: 2.0,
                ..Default::default()
            },
        )?);
        let axes = Node::new_mesh(axes);
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
        let grid = Node::new_mesh(grid);
        grid.rotate_x(-Angle::RIGHT, Transform::default());
        scene.add_child(&grid);

        Ok(Box::new(MovementRigExample {
            renderer,
            scene,
            camera,
            rig,
        }))
    }
}

impl Application for MovementRigExample {
    fn update(&mut self, key_state: &KeyState) {
        self.rig.update(key_state)
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}
