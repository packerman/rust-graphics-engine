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
        math::angle::Angle,
    },
    core::{
        camera::Camera,
        geometry::Geometry,
        material::Material,
        mesh::Mesh,
        node::Node,
        render_target::RenderTarget,
        renderer::{self, Renderer, RendererOptions},
        texture::{Texture, TextureData, TextureUnit},
        uniform::data::Sampler2D,
    },
    extras::{
        effects::{self, Blend, Blur},
        postprocessor::Postprocessor,
    },
    geometry::{parametric::Sphere, Rectangle},
    material::{
        self,
        basic::{BasicMaterial, SurfaceMaterial},
        texture::TextureMaterial,
    },
};

struct Example {
    glow_pass: Postprocessor,
    combo_pass: Postprocessor,
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

        let glow_scene = Node::new_group();
        {
            let glow_sphere = Node::new_mesh(Mesh::initialize(
                context,
                Rc::new(Geometry::from_with_context(context, Sphere::default())?),
                Rc::new(Material::from_with_context(
                    context,
                    SurfaceMaterial {
                        basic: BasicMaterial {
                            base_color: color::red(),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                )?),
            )?);
            glow_sphere.set_transform(&sphere.transform());
            glow_scene.add_child(&glow_sphere);
        }

        let resolution = renderer::get_canvas_resolution(context);

        let glow_target = RenderTarget::initialize(context, resolution)?;
        let glow_texture = glow_target.texture();
        let mut glow_pass = Postprocessor::initialize(
            context,
            Rc::clone(&renderer),
            glow_scene,
            Rc::clone(&camera),
            Some(glow_target),
            TextureUnit::from(3),
        )?;
        glow_pass.add_effect(context, |sampler| {
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
        glow_pass.add_effect(context, |sampler| {
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

        let mut combo_pass = Postprocessor::initialize(
            context,
            renderer,
            scene,
            camera,
            None,
            TextureUnit::from(4),
        )?;
        combo_pass.add_effect(context, |sampler| {
            effects::additive_blend(
                context,
                sampler,
                Sampler2D::new(Rc::clone(&glow_texture), TextureUnit::from(5)),
                Blend {
                    original_strength: 1.0,
                    blend_strength: 3.0,
                },
            )
        })?;

        Ok(Box::new(Example {
            glow_pass,
            combo_pass,
        }))
    }
}

impl Application for Example {
    fn update(&mut self, _key_state: &KeyState) {}

    fn render(&self, context: &WebGl2RenderingContext) {
        self.glow_pass.render(context);
        self.combo_pass.render(context);
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
