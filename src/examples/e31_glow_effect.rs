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
        render_target::RenderTarget,
        renderer::{self, Renderer, RendererOptions},
        texture::Sampler2D,
    },
    core::{
        camera::{Camera, Perspective},
        material::Material,
        mesh::Mesh,
        node::Node,
        scene::Scene,
        texture::{Texture, TextureUnit},
    },
    extras::{
        effects::{self, Blend, Blur},
        postprocessor::Postprocessor,
    },
    geometry::{parametric::Sphere, rectangle::Rectangle},
    material::{
        self,
        basic::{BasicMaterial, SurfaceMaterial},
    },
};

struct Example {
    glow_pass: Postprocessor,
    combo_pass: Postprocessor,
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
            scene.add_node(Rc::clone(&sphere));
        }

        let mut glow_scene = Scene::new_empty();
        {
            let glow_sphere = Node::new_with_mesh(Mesh::initialize(
                context,
                &Geometry::from_with_context(context, Sphere::default())?,
                <Rc<Material>>::from_with_context(
                    context,
                    SurfaceMaterial {
                        basic: BasicMaterial {
                            base_color: color::red(),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                )?,
            )?);
            glow_sphere
                .borrow_mut()
                .set_local_transform(sphere.borrow().local_transform());
            glow_scene.add_node(glow_sphere);
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
            TextureUnit(3),
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

        let mut combo_pass =
            Postprocessor::initialize(context, renderer, scene, camera, None, TextureUnit(4))?;
        combo_pass.add_effect(context, |sampler| {
            effects::additive_blend(
                context,
                sampler,
                Sampler2D::new(Rc::clone(&glow_texture), TextureUnit(5)),
                Blend {
                    original_strength: 1.0,
                    blend_strength: 3.0,
                },
            )
        })?;

        Ok(Box::new(Example {
            glow_pass,
            combo_pass,
            lights: Lights::default(),
        }))
    }
}

impl Application for Example {
    fn name(&self) -> &str {
        "Glow effect"
    }

    fn update(&mut self, _key_state: &KeyState) {}

    fn render(&self, context: &WebGl2RenderingContext) {
        self.glow_pass.render(context, &self.lights);
        self.combo_pass.render(context, &self.lights);
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
