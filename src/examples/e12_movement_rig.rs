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
    core::{
        camera::{Camera, Perspective},
        mesh::Mesh,
        node::Node,
        scene::Scene,
    },
    extras::{
        axes_helper::AxesHelper, camera_controller::CameraController, grid_helper::GridHelper,
    },
    legacy::renderer::Renderer,
};

struct Example {
    renderer: Renderer,
    scene: Scene,
    camera: Rc<RefCell<Camera>>,
    controller: CameraController,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        let renderer = Renderer::initialize(context, Default::default(), None);
        let mut scene = Scene::new_empty();

        let camera = Camera::new(Perspective::default());
        let camera_node = Node::new_with_camera(Rc::clone(&camera));
        scene.add_node(camera_node);

        let controller = CameraController::make_for_camera(&camera)
            .expect("Camera controller should be created.");
        controller.set_position(&glm::vec3(0.5, 1.0, 5.0));

        let axes = <Rc<Mesh>>::from_with_context(
            context,
            AxesHelper {
                axis_length: 2.0,
                ..Default::default()
            },
        )?;
        let axes = Node::new_with_mesh(axes);
        scene.add_node(axes);

        let grid = <Rc<Mesh>>::from_with_context(
            context,
            GridHelper {
                size: 20.0,
                grid_color: color::white(),
                center_color: color::yellow(),
                ..Default::default()
            },
        )?;
        let grid = Node::new_with_mesh(grid);
        grid.borrow_mut().rotate_x(-Angle::RIGHT);
        scene.add_node(grid);

        Ok(Box::new(Example {
            renderer,
            scene,
            camera,
            controller,
        }))
    }
}

impl Application for Example {
    fn name(&self) -> &str {
        "Camera controller"
    }

    fn update(&mut self, key_state: &KeyState) {
        self.controller.update(key_state)
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
