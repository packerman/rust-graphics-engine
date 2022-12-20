use std::rc::Rc;

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::{
    base::{
        application::{self, Application, AsyncCreator},
        color,
        convert::FromWithContext,
        input::KeyState,
    },
    core::{
        camera::Camera,
        geometry::Geometry,
        math::angle::Angle,
        mesh::Mesh,
        node::Node,
        renderer::{Renderer, RendererOptions},
        texture::{Texture, TextureData, TextureUnit},
        uniform::data::Sampler2D,
    },
    extras::{
        effects::{self, Blend, Blur, BrightFilter},
        postprocessor::Postprocessor,
    },
    geometry::{parametric::Sphere, Rectangle},
    material::{self, texture::TextureMaterial},
};

struct Example {
    postprocessor: Postprocessor,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        let renderer = Rc::new(Renderer::initialize(
            context,
            RendererOptions {
                clear_color: color::black(),
                ..Default::default()
            },
            None,
        ));
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

        let mut postprocessor = Postprocessor::initialize(
            context,
            renderer,
            scene,
            camera,
            None,
            TextureUnit::from(3),
        )?;
        postprocessor.add_effect(context, |sampler| {
            effects::bright_filter(context, sampler, BrightFilter { threshold: 2.4 })
        })?;
        postprocessor.add_effect(context, |sampler| {
            let texture_size = sampler.resolution();
            effects::horizontal_blur(
                context,
                sampler,
                Blur {
                    texture_size,
                    blur_radius: 50,
                },
            )
        })?;
        postprocessor.add_effect(context, |sampler| {
            let texture_size = sampler.resolution();
            effects::vertical_blur(
                context,
                sampler,
                Blur {
                    texture_size,
                    blur_radius: 50,
                },
            )
        })?;
        if let Some(main_scene) = postprocessor.get_texture(0) {
            postprocessor.add_effect(context, |sampler| {
                effects::additive_blend(
                    context,
                    sampler,
                    Sampler2D::new(Rc::clone(&main_scene), TextureUnit::from(4)),
                    Blend {
                        original_strength: 2.0,
                        blend_strength: 1.0,
                    },
                )
            })?;
        }

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
