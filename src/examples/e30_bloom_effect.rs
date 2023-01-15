use std::rc::Rc;

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::{
    api::geometry::Geometry,
    base::{
        application::{self, Application, AsyncCreator},
        color,
        convert::FromWithContext,
        input::KeyState,
        math::angle::Angle,
    },
    classic::{
        light::Lights,
        renderer::{Renderer, RendererOptions},
        texture::Sampler2D,
    },
    core::{
        camera::{Camera, Perspective},
        mesh::Mesh,
        node::Node,
        scene::Scene,
        texture::{Texture, TextureUnit},
    },
    extras::{
        effects::{self, Blend, Blur, BrightFilter},
        postprocessor::Postprocessor,
    },
    geometry::{parametric::Sphere, rectangle::Rectangle},
    material,
};

struct Example {
    postprocessor: Postprocessor,
    lights: Lights,
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
        let mut scene = Scene::new_empty();

        let camera = Camera::new(Perspective::default());
        {
            let camera = Node::new_with_camera(Rc::clone(&camera));
            camera.borrow_mut().set_position(&glm::vec3(0.0, 1.0, 4.0));
            scene.add_node(camera);
        }
        {
            let sky = Node::new_with_mesh(Mesh::initialize(
                context,
                &Geometry::from_with_context(
                    context,
                    Sphere {
                        radius: 50.0,
                        ..Default::default()
                    },
                )?,
                material::texture::create(
                    context,
                    Texture::fetch(context, "images/sky-earth.jpg").await?,
                    TextureUnit(0),
                    Default::default(),
                )?,
            )?);
            scene.add_node(sky);
        }
        {
            let grass = Node::new_with_mesh(Mesh::initialize(
                context,
                &Geometry::from_with_context(
                    context,
                    Rectangle {
                        width: 100.0,
                        height: 100.0,
                        ..Default::default()
                    },
                )?,
                material::texture::create(
                    context,
                    Texture::fetch(context, "images/grass.jpg").await?,
                    TextureUnit(1),
                    material::texture::Properties {
                        repeat_uv: glm::vec2(50.0, 50.0),
                        ..Default::default()
                    },
                )?,
            )?);
            grass.borrow_mut().rotate_x(-Angle::RIGHT);
            scene.add_node(grass);
        }
        let sphere = Node::new_with_mesh(Mesh::initialize(
            context,
            &Geometry::from_with_context(context, Sphere::default())?,
            material::texture::create(
                context,
                Texture::fetch(context, "images/grid.png").await?,
                TextureUnit(2),
                Default::default(),
            )?,
        )?);
        {
            sphere.borrow_mut().set_position(&glm::vec3(0.0, 1.0, 0.0));
            scene.add_node(sphere);
        }

        let mut postprocessor =
            Postprocessor::initialize(context, renderer, scene, camera, None, TextureUnit(3))?;
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
                    Sampler2D::new(Rc::clone(&main_scene), TextureUnit(4)),
                    Blend {
                        original_strength: 2.0,
                        blend_strength: 1.0,
                    },
                )
            })?;
        }

        Ok(Box::new(Example {
            postprocessor,
            lights: Lights::default(),
        }))
    }
}

impl Application for Example {
    fn name(&self) -> &str {
        "Bloom effect"
    }

    fn update(&mut self, _key_state: &KeyState) {}

    fn render(&self, context: &WebGl2RenderingContext) {
        self.postprocessor.render(context, &self.lights);
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
