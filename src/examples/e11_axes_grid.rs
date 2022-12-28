use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::{
    base::{
        application::{self, Application, AsyncCreator},
        color,
        convert::FromWithContext,
        input::KeyState,
        math::angle::Angle,
    },
    extras::{axes_helper::AxesHelper, grid_helper::GridHelper},
    legacy::{
        camera::Camera,
        mesh::Mesh,
        node::{Node, Transform},
        renderer::{Renderer, RendererOptions},
    },
};

struct Example {
    renderer: Renderer,
    scene: Rc<Node>,
    camera: Rc<RefCell<Camera>>,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        let renderer = Renderer::initialize(context, RendererOptions::default(), None);
        let scene = Node::new_group();

        let camera = Camera::new_perspective(Default::default());
        let camera_node = Node::new_camera(Rc::clone(&camera));
        camera_node.set_position(&glm::vec3(0.5, 1.0, 5.0));
        scene.add_child(&camera_node);

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
                grid_color: color::white(),
                center_color: color::yellow(),
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
        }))
    }
}

impl Application for Example {
    fn update(&mut self, _key_state: &KeyState) {}

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
