use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::core::{
    application::{Application, AsyncCreator},
    camera::Camera,
    convert::FromWithContext,
    geometry::{parametric::Sphere, Geometry, Rectangle},
    input::KeyState,
    material::{self, texture::TextureMaterial},
    matrix::Angle,
    mesh::Mesh,
    node::Node,
    renderer::{Renderer, RendererOptions},
    texture::{Texture, TextureUnit},
};

pub struct Example {
    renderer: Renderer,
    scene: Rc<Node>,
    rig: Rc<Node>,
    camera: Rc<RefCell<Camera>>,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Self> {
        let renderer = Renderer::new_initialized(context, RendererOptions::default());
        let scene = Node::new_group();

        let camera = Rc::new(RefCell::new(Camera::default()));
        let rig = Node::new_movement_rig(Default::default());
        {
            let camera = Node::new_camera(Rc::clone(&camera));
            rig.add_child(&camera);
            scene.add_child(&rig);
            rig.set_position(&glm::vec3(0.0, 1.0, 4.0));
        }
        {
            let geometry = Geometry::from_with_context(
                context,
                Sphere {
                    radius: 50.0,
                    ..Default::default()
                },
            )?;
            let material = Rc::new(material::texture::create(
                context,
                Texture::load_from_source(
                    context,
                    "images/set01/sky-earth.jpg",
                    Default::default(),
                )
                .await?,
                TextureUnit::from(0),
                Default::default(),
            )?);
            let sky = Node::new_mesh(Box::new(Mesh::new(context, geometry, material)?));
            scene.add_child(&sky);
        }
        {
            let geometry = Geometry::from_with_context(
                context,
                Rectangle {
                    width: 100.0,
                    height: 100.0,
                },
            )?;
            let material = Rc::new(material::texture::create(
                context,
                Texture::load_from_source(context, "images/set01/grass.jpg", Default::default())
                    .await?,
                TextureUnit::from(1),
                TextureMaterial {
                    repeat_uv: glm::vec2(50.0, 50.0),
                    ..Default::default()
                },
            )?);
            let grass = Node::new_mesh(Box::new(Mesh::new(context, geometry, material)?));
            grass.rotate_x(-Angle::RIGHT, Default::default());
            scene.add_child(&grass);
        }

        Ok(Example {
            renderer,
            rig,
            scene,
            camera,
        })
    }
}

impl Application for Example {
    fn update(&mut self, key_state: &KeyState) {
        self.rig.update(key_state);
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}
