use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::{
    api::geometry::Geometry,
    base::{
        application::{self, Application, AsyncCreator},
        input::KeyState,
    },
    core::{
        camera::Camera,
        node::Node,
        texture::{Texture, TextureUnit},
    },
    geometry::Rectangle,
    legacy::renderer::Renderer,
    material,
};

struct Example {
    renderer: Renderer,
    scene: Rc<Node>,
    camera: Rc<RefCell<Camera>>,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        let renderer = Renderer::initialize(context, Default::default(), None);
        let scene = Node::new_group();

        let camera = Camera::new_perspective(Default::default());
        let camera_node = Node::new_camera(Rc::clone(&camera));
        camera_node.set_position(&glm::vec3(0.0, 0.0, 1.0));
        scene.add_child(&camera_node);

        let geometry = Rc::new(Geometry::from_with_context(context, Rectangle::default())?);
        let material = material::texture::create(
            context,
            Rc::new(Texture::fetch(context, "images/grid.png")?),
            TextureUnit(0),
            Default::default(),
        )?;
        let mesh = Node::new_mesh(geometry.create_mesh(context, material)?);
        scene.add_child(&mesh);
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
