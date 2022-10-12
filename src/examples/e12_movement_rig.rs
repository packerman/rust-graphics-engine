use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::core::{
    application::{self, Application, AsyncCreator},
    camera::Camera,
    color::Color,
    convert::FromWithContext,
    extras::{AxesHelper, GridHelper},
    input::KeyState,
    matrix::Angle,
    mesh::Mesh,
    node::{Node, Transform},
    renderer::Renderer,
};

struct Example {
    renderer: Renderer,
    scene: Rc<Node>,
    camera: Rc<RefCell<Camera>>,
    rig: Rc<Node>,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        let renderer = Renderer::new(context, Default::default());
        let scene = Node::new_group();

        let camera = Camera::new_perspective(Default::default());
        let camera_node = Node::new_camera(Rc::clone(&camera));

        let rig = Node::new_movement_rig(Default::default());
        rig.add_child(&camera_node);
        rig.set_position(&glm::vec3(0.5, 1.0, 5.0));
        scene.add_child(&rig);

        let axes = Mesh::from_with_context(
            context,
            AxesHelper {
                axis_length: 2.0,
                ..Default::default()
            },
        )?;
        let axes = Node::new_mesh(axes);
        scene.add_child(&axes);

        let grid = Mesh::from_with_context(
            context,
            GridHelper {
                size: 20.0,
                grid_color: Color::white(),
                center_color: Color::yellow(),
                ..Default::default()
            },
        )?;
        let grid = Node::new_mesh(grid);
        grid.rotate_x(-Angle::RIGHT, Transform::default());
        scene.add_child(&grid);

        Ok(Box::new(Example {
            renderer,
            scene,
            camera,
            rig,
        }))
    }
}

impl Application for Example {
    fn update(&mut self, key_state: &KeyState) {
        self.rig.update(key_state)
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
