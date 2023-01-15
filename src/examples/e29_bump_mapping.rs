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
        util::shared_ref::SharedRef,
    },
    classic::{
        light::{Light, Lights},
        renderer::Renderer,
        texture::Sampler2D,
    },
    core::{
        camera::{Camera, Perspective},
        mesh::Mesh,
        node::Node,
        scene::Scene,
        texture::{Texture, TextureUnit},
    },
    geometry::rectangle::Rectangle,
    material::{self, lambert::LambertMaterial},
};

struct Example {
    renderer: Renderer,
    scene: Scene,
    camera: SharedRef<Camera>,
    lights: Lights,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        let renderer = Renderer::initialize(context, Default::default(), None);
        let mut scene = Scene::new_empty();
        let mut lights = Lights::new();

        let camera = Camera::new(Perspective::default());
        {
            let camera = Node::new_with_camera(Rc::clone(&camera));
            camera.borrow_mut().set_position(&glm::vec3(0.0, 0.0, 2.5));
            scene.add_node(camera);
        }

        let point = lights.create_node(Light::point(color::white(), glm::vec3(1.2, 1.2, 0.3)));
        point.add_to_scene(&mut scene);

        {
            let mesh = Node::new_with_mesh(Mesh::initialize(
                context,
                &Geometry::from_with_context(
                    context,
                    Rectangle {
                        width: 2.0,
                        height: 2.0,
                        ..Default::default()
                    },
                )?,
                material::lambert::create(
                    context,
                    LambertMaterial {
                        ambient: color::rgb(0.3, 0.3, 0.3),
                        texture: Sampler2D::new(
                            Texture::fetch(context, "images/brick-color.png").await?,
                            TextureUnit(0),
                        )
                        .into(),
                        bump_texture: Sampler2D::new(
                            Texture::fetch(context, "images/brick-bump.png").await?,
                            TextureUnit(1),
                        )
                        .into(),
                        ..Default::default()
                    },
                )?,
            )?);
            scene.add_node(mesh);
        }

        Ok(Box::new(Example {
            renderer,
            scene,
            camera,
            lights,
        }))
    }
}

impl Application for Example {
    fn name(&self) -> &str {
        "Bump mapping"
    }

    fn update(&mut self, _key_state: &KeyState) {}

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer
            .render_with_lights(context, &self.scene, &self.camera, &self.lights);
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
