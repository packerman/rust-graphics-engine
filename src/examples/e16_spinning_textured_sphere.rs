use std::{cell::RefCell, f32::consts::TAU, rc::Rc};

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::core::{
    application::{self, Application, AsyncCreator},
    camera::Camera,
    convert::FromWithContext,
    geometry::{parametric::Sphere, Geometry},
    input::KeyState,
    material,
    matrix::Angle,
    mesh::Mesh,
    node::{Node, Transform},
    renderer::{Renderer, RendererOptions},
    texture::{Texture, TextureData, TextureUnit},
};

struct Example {
    renderer: Renderer,
    scene: Rc<Node>,
    mesh: Rc<Node>,
    camera: Rc<RefCell<Camera>>,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Self> {
        let renderer = Renderer::new(context, RendererOptions::default());
        let scene = Node::new_group();

        let camera = Rc::new(RefCell::new(Camera::default()));
        {
            let camera = Node::new_camera(Rc::clone(&camera));
            camera.set_position(&glm::vec3(0.0, 0.0, 2.1));
            scene.add_child(&camera);
        }

        let geometry = Geometry::from_with_context(
            context,
            Sphere {
                radius_segments: 64,
                height_segments: 64,
                ..Default::default()
            },
        )?;
        let material = material::texture::create(
            context,
            Texture::initialize(
                context,
                TextureData::load_from_source("images/earth.jpg").await?,
                Default::default(),
            )?,
            TextureUnit::from(0),
            Default::default(),
        )?;
        let mesh = Box::new(Mesh::new(context, geometry, material)?);
        let mesh = Node::new_mesh(mesh);
        scene.add_child(&mesh);

        Ok(Example {
            renderer,
            mesh,
            scene,
            camera,
        })
    }
}

impl Application for Example {
    fn update(&mut self, _key_state: &KeyState) {
        self.mesh
            .rotate_y(Angle::from_radians(TAU) / 500.0, Transform::Local);
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
