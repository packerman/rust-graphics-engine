use std::{cell::RefCell, f32::consts::TAU, rc::Rc};

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::{
    base::{
        application::{self, Application, AsyncCreator},
        convert::FromWithContext,
        input::KeyState,
    },
    core::{
        camera::Camera,
        geometry::Geometry,
        math::angle::Angle,
        mesh::Mesh,
        node::{Node, Transform},
        renderer::{Renderer, RendererOptions},
        texture::{Texture, TextureData, TextureUnit},
    },
    geometry::BoxGeometry,
    material,
};

struct Example {
    renderer: Renderer,
    scene: Rc<Node>,
    mesh: Rc<Node>,
    camera: Rc<RefCell<Camera>>,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        let renderer = Renderer::initialize(context, RendererOptions::default(), None);
        let scene = Node::new_group();

        let camera = Camera::new_perspective(Default::default());
        let camera_node = Node::new_camera(Rc::clone(&camera));
        camera_node.set_position(&glm::vec3(0.0, 0.0, 1.8));
        scene.add_child(&camera_node);

        let geometry = Rc::new(Geometry::from_with_context(
            context,
            BoxGeometry::default(),
        )?);
        let material = material::texture::create(
            context,
            Texture::initialize(
                context,
                TextureData::load_from_source("images/crate.png").await?,
                Default::default(),
            )?,
            TextureUnit::from(0),
            Default::default(),
        )?;
        let mesh = Mesh::initialize(context, geometry, material)?;
        let mesh = Node::new_mesh(mesh);
        scene.add_child(&mesh);

        Ok(Box::new(Example {
            renderer,
            mesh,
            scene,
            camera,
        }))
    }
}

impl Application for Example {
    fn update(&mut self, _key_state: &KeyState) {
        self.mesh
            .rotate_y(Angle::from_radians(TAU) / 450.0, Transform::Local);
        self.mesh
            .rotate_x(Angle::from_radians(TAU) / 600.0, Transform::Local);
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
