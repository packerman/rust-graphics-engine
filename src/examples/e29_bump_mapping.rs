use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::{
    api::geometry::Geometry,
    base::{
        application::{self, Application, AsyncCreator},
        color,
        input::KeyState,
    },
    core::{
        camera::Camera,
        mesh::Mesh,
        node::Node,
        texture::{Texture, TextureUnit},
    },
    geometry::Rectangle,
    legacy::{light::Light, renderer::Renderer, texture::Sampler2D},
    material::{self, lambert::LambertMaterial},
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
        {
            let camera = Node::new_camera(Rc::clone(&camera));
            camera.set_position(&glm::vec3(0.0, 0.0, 2.5));
            scene.add_child(&camera);
        }

        let point = Node::new_light(Light::point(color::white(), glm::vec3(1.2, 1.2, 0.3)));

        scene.add_child(&point);

        {
            let mesh = Node::new_mesh(Mesh::initialize(
                context,
                Rc::new(Geometry::from_with_context(
                    context,
                    Rectangle {
                        width: 2.0,
                        height: 2.0,
                        ..Default::default()
                    },
                )?),
                material::lambert::create(
                    context,
                    LambertMaterial {
                        ambient: color::rgb(0.3, 0.3, 0.3),
                        texture: Sampler2D::new(
                            Texture::fetch(context, "images/brick-color.png")?,
                            TextureUnit(0),
                        )
                        .into(),
                        bump_texture: Sampler2D::new(
                            Texture::initialize(context, "images/brick-bump.png")?,
                            TextureUnit(1),
                        )
                        .into(),
                        ..Default::default()
                    },
                )?,
            )?);
            scene.add_child(&mesh);
        }

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
