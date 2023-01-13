use std::{cell::RefCell, f32::consts::TAU, rc::Rc};

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
    classic::renderer::{Renderer, RendererOptions},
    core::{
        camera::{Camera, Perspective},
        mesh::Mesh,
        node::Node,
        scene::Scene,
        texture::{Texture, TextureUnit},
    },
    geometry::parametric::Sphere,
    material,
};

struct Example {
    renderer: Renderer,
    scene: Scene,
    mesh: SharedRef<Node>,
    camera: Rc<RefCell<Camera>>,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        let renderer = Renderer::initialize(context, RendererOptions::default(), None);
        let mut scene = Scene::new_empty();

        let camera = Camera::new(Perspective::default());
        {
            let camera = Node::new_with_camera(Rc::clone(&camera));
            camera.borrow_mut().set_position(&glm::vec3(0.0, 0.0, 2.1));
            scene.add_node(camera);
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
            Texture::fetch(context, "images/earth.jpg").await?,
            TextureUnit(0),
            Default::default(),
        )?;
        let mesh = Mesh::initialize(context, &geometry, material)?;
        let mesh = Node::new_with_mesh(mesh);
        scene.add_node(Rc::clone(&mesh));

        Ok(Box::new(Example {
            renderer,
            mesh,
            scene,
            camera,
        }))
    }
}

impl Application for Example {
    fn name(&self) -> &str {
        "Spinning textured sphere"
    }

    fn update(&mut self, _key_state: &KeyState) {
        self.mesh
            .borrow_mut()
            .rotate_y(Angle::from_radians(TAU) / 500.0);
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
