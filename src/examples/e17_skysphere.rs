use std::rc::Rc;

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::{
    api::geometry::Geometry,
    base::{
        application::{self, Application, AsyncCreator},
        convert::FromWithContext,
        input::KeyState,
        math::angle::Angle,
        util::shared_ref::SharedRef,
    },
    core::{
        camera::{Camera, Perspective},
        mesh::Mesh,
        node::Node,
        scene::Scene,
        texture::{Texture, TextureUnit},
    },
    extras::camera_controller::CameraController,
    geometry::{parametric::Sphere, Rectangle},
    legacy::renderer::{Renderer, RendererOptions},
    material,
};

struct Example {
    renderer: Renderer,
    scene: Scene,
    controller: CameraController,
    camera: SharedRef<Camera>,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        let renderer = Renderer::initialize(context, RendererOptions::default(), None);
        let mut scene = Scene::new_empty();

        let camera = Camera::new(Perspective::default());
        {
            let camera = Node::new_with_camera(Rc::clone(&camera));
            scene.add_root_node(Rc::clone(&camera));
        }
        let controller = CameraController::make_for_camera(&camera)
            .expect("Camera controller should be created");
        controller.set_position(&glm::vec3(0.0, 1.0, 4.0));
        {
            let geometry = Geometry::from_with_context(
                context,
                Sphere {
                    radius: 50.0,
                    ..Default::default()
                },
            )?;
            let material = material::texture::create(
                context,
                Texture::fetch(context, "images/sky-earth.jpg").await?,
                TextureUnit(0),
                Default::default(),
            )?;
            let sky = Node::new_with_mesh(Mesh::initialize(context, &geometry, material)?);
            scene.add_root_node(sky);
        }
        {
            let geometry = Geometry::from_with_context(
                context,
                Rectangle {
                    width: 100.0,
                    height: 100.0,
                    ..Default::default()
                },
            )?;
            let material = material::texture::create(
                context,
                Texture::fetch(context, "images/grass.jpg").await?,
                TextureUnit(1),
                material::texture::Properties {
                    repeat_uv: glm::vec2(50.0, 50.0),
                    ..Default::default()
                },
            )?;
            let grass = Node::new_with_mesh(Mesh::initialize(context, &geometry, material)?);
            grass.borrow_mut().rotate_x(-Angle::RIGHT);
            scene.add_root_node(grass);
        }

        Ok(Box::new(Example {
            renderer,
            controller,
            scene,
            camera,
        }))
    }
}

impl Application for Example {
    fn name(&self) -> &str {
        "Sky sphere"
    }

    fn update(&mut self, key_state: &KeyState) {
        self.controller.update(key_state);
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
