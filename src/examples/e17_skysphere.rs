use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::{
    base::{
        application::{self, Application, AsyncCreator},
        convert::FromWithContext,
        input::KeyState,
        math::angle::Angle,
    },
    core::{
        camera::Camera,
        geometry::Geometry,
        mesh::Mesh,
        node::Node,
        renderer::{Renderer, RendererOptions},
        texture::{Texture, TextureData, TextureUnit},
    },
    geometry::{parametric::Sphere, Rectangle},
    material::{self, texture::TextureMaterial},
};

struct Example {
    renderer: Renderer,
    scene: Rc<Node>,
    rig: Rc<Node>,
    camera: Rc<RefCell<Camera>>,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        let renderer = Renderer::initialize(context, RendererOptions::default(), None);
        let scene = Node::new_group();

        let camera = Camera::new_perspective(Default::default());
        let rig = Node::new_movement_rig(Default::default());
        {
            let camera = Node::new_camera(Rc::clone(&camera));
            rig.add_child(&camera);
            scene.add_child(&rig);
            rig.set_position(&glm::vec3(0.0, 1.0, 4.0));
        }
        {
            let geometry = Rc::new(Geometry::from_with_context(
                context,
                Sphere {
                    radius: 50.0,
                    ..Default::default()
                },
            )?);
            let material = material::texture::create(
                context,
                Texture::initialize(
                    context,
                    TextureData::load_from_source("images/sky-earth.jpg").await?,
                    Default::default(),
                )?,
                TextureUnit::from(0),
                Default::default(),
            )?;
            let sky = Node::new_mesh(Mesh::initialize(context, geometry, material)?);
            scene.add_child(&sky);
        }
        {
            let geometry = Rc::new(Geometry::from_with_context(
                context,
                Rectangle {
                    width: 100.0,
                    height: 100.0,
                    ..Default::default()
                },
            )?);
            let material = material::texture::create(
                context,
                Texture::initialize(
                    context,
                    TextureData::load_from_source("images/grass.jpg").await?,
                    Default::default(),
                )?,
                TextureUnit::from(1),
                TextureMaterial {
                    repeat_uv: glm::vec2(50.0, 50.0),
                    ..Default::default()
                },
            )?;
            let grass = Node::new_mesh(Mesh::initialize(context, geometry, material)?);
            grass.rotate_x(-Angle::RIGHT, Default::default());
            scene.add_child(&grass);
        }

        Ok(Box::new(Example {
            renderer,
            rig,
            scene,
            camera,
        }))
    }
}

impl Application for Example {
    fn update(&mut self, key_state: &KeyState) {
        self.rig.update_key_state(key_state);
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
