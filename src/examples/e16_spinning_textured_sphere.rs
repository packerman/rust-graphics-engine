use std::{cell::RefCell, f32::consts::TAU, rc::Rc};

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::{
    api::geometry::Geometry,
    base::{
        application::{self, Application, AsyncCreator},
        input::KeyState,
        math::angle::Angle,
    },
    core::{
        camera::Camera,
        mesh::Mesh,
        node::Node,
        texture::{Texture, TextureUnit},
    },
    geometry::parametric::Sphere,
    legacy::renderer::{Renderer, RendererOptions},
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
        {
            let camera = Node::new_camera(Rc::clone(&camera));
            camera.set_position(&glm::vec3(0.0, 0.0, 2.1));
            scene.add_child(&camera);
        }

        let geometry = Rc::new(Geometry::from_with_context(
            context,
            Sphere {
                radius_segments: 64,
                height_segments: 64,
                ..Default::default()
            },
        )?);
        let material = material::texture::create(
            context,
            Texture::fetch(context, "images/earth.jpg")?,
            TextureUnit(0),
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
        self.mesh.rotate_y(Angle::from_radians(TAU) / 500.0);
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
