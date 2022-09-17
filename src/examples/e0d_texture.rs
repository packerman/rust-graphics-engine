use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::core::{
    application::{Application, AsyncCreator},
    camera::Camera,
    convert::FromWithContext,
    geometry::{Geometry, Rectangle},
    input::KeyState,
    material,
    mesh::Mesh,
    node::Node,
    renderer::Renderer,
    texture::{Texture, TextureUnit},
};

pub struct Example {
    renderer: Renderer,
    scene: Rc<Node>,
    camera: Rc<RefCell<Camera>>,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Self> {
        let renderer = Renderer::new_initialized(context, Default::default());
        let scene = Node::new_group();

        let camera = Rc::new(RefCell::new(Camera::default()));
        let camera_node = Node::new_camera(Rc::clone(&camera));
        camera_node.set_position(&glm::vec3(0.0, 0.0, 1.0));
        scene.add_child(&camera_node);

        let geometry = Geometry::from_with_context(context, Rectangle::default())?;
        let material = material::texture::create(
            context,
            Texture::load_from_source(context, "images/set01/grid.png", Default::default()).await?,
            TextureUnit::from(0),
            Default::default(),
        )?;
        let mesh = Node::new_mesh(Box::new(Mesh::new(context, geometry, Rc::new(material))?));
        scene.add_child(&mesh);
        Ok(Example {
            renderer,
            scene,
            camera,
        })
    }
}

impl Application for Example {
    fn update(&mut self, _key_state: &KeyState) {}

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}
