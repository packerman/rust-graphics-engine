use std::rc::Rc;

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::{
    core::{
        application::{self, Application, AsyncCreator},
        camera::Camera,
        color::Color,
        convert::FromWithContext,
        geometry::{Geometry, Rectangle},
        input::KeyState,
        math::angle::Angle,
        mesh::Mesh,
        node::Node,
        renderer::{Renderer, RendererOptions},
        texture::{Texture, TextureData, TextureUnit},
    },
    extras::{effects, postprocessor::Postprocessor},
    geometry::parametric::Sphere,
    material::{self, texture::TextureMaterial},
};

struct Example {
    postprocessor: Postprocessor,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        let renderer = Renderer::new(context, RendererOptions::default());
        let scene = Node::new_group();

        let camera = Camera::new_perspective(Default::default());
        {
            let camera = Node::new_camera(Rc::clone(&camera));
            scene.add_child(&camera);
            camera.set_position(&glm::vec3(0.0, 1.0, 4.0));
        }
        {
            let sky = Node::new_mesh(Mesh::initialize(
                context,
                Rc::new(Geometry::from_with_context(
                    context,
                    Sphere {
                        radius: 50.0,
                        ..Default::default()
                    },
                )?),
                material::texture::create(
                    context,
                    Texture::initialize(
                        context,
                        TextureData::load_from_source("images/sky-earth.jpg").await?,
                        Default::default(),
                    )?,
                    TextureUnit::from(0),
                    Default::default(),
                )?,
            )?);
            scene.add_child(&sky);
        }
        {
            let grass = Node::new_mesh(Mesh::initialize(
                context,
                Rc::new(Geometry::from_with_context(
                    context,
                    Rectangle {
                        width: 100.0,
                        height: 100.0,
                        ..Default::default()
                    },
                )?),
                material::texture::create(
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
                )?,
            )?);
            grass.rotate_x(-Angle::RIGHT, Default::default());
            scene.add_child(&grass);
        }
        let sphere = Node::new_mesh(Mesh::initialize(
            context,
            Rc::new(Geometry::from_with_context(context, Sphere::default())?),
            material::texture::create(
                context,
                Texture::initialize(
                    context,
                    TextureData::load_from_source("images/grid.png").await?,
                    Default::default(),
                )?,
                TextureUnit::from(2),
                Default::default(),
            )?,
        )?);
        {
            sphere.set_position(&glm::vec3(0.0, 1.0, 0.0));
            scene.add_child(&sphere);
        }

        let postprocessor = Postprocessor::initialize(
            context,
            renderer,
            scene,
            camera,
            None,
            vec![
                &|sampler| effects::tint(context, sampler, Color::lime()),
                &|sampler| effects::color_reduce(context, sampler, 5),
                &|sampler| effects::pixelate(context, sampler, 4, glm::vec2(800.0, 600.0)),
            ],
            TextureUnit::from(3),
        )?;

        Ok(Box::new(Example { postprocessor }))
    }
}

impl Application for Example {
    fn update(&mut self, _key_state: &KeyState) {}

    fn render(&self, context: &WebGl2RenderingContext) {
        self.postprocessor.render(context);
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
