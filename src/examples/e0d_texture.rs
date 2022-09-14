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
    material::{self, texture::TextureMaterial},
    mesh::Mesh,
    node::Node,
    renderer::Renderer,
    texture::{self, Texture, TextureUnit},
};

pub struct TextureExample {
    renderer: Renderer,
    scene: Rc<Node>,
    camera: Rc<RefCell<Camera>>,
}

#[async_trait(?Send)]
impl AsyncCreator for TextureExample {
    async fn create(context: &WebGl2RenderingContext) -> Result<Self> {
        let renderer = Renderer::new_initialized(context, Default::default());
        let scene = Node::new_group();

        let camera = Rc::new(RefCell::new(Camera::default()));
        let camera_node = Node::new_camera(Rc::clone(&camera));
        camera_node.set_position(&glm::vec3(0.0, 0.0, 1.0));
        scene.add_child(&camera_node);

        let geometry = Geometry::from_with_context(context, Rectangle::default())?;
        let image = texture::load_image("images/set02/grid.png").await?;
        let texture = Texture::new_initialized(context, image, texture::Properties::default())?;
        let material = material::texture::texture_material(
            context,
            texture,
            TextureUnit::from(0),
            TextureMaterial::default(),
        )?;
        let mesh = Node::new_mesh(Box::new(Mesh::new(context, geometry, material)?));
        scene.add_child(&mesh);
        Ok(TextureExample {
            renderer,
            scene,
            camera,
        })
    }
}

impl Application for TextureExample {
    fn update(&mut self, _key_state: &KeyState) {}

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}
