use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::{
    base::{
        application::{self, Application, AsyncCreator},
        convert::FromWithContext,
        input::KeyState,
        math::{angle::Angle, matrix},
    },
    core::{
        camera::Camera,
        geometry::Geometry,
        mesh::Mesh,
        node::Node,
        renderer::Renderer,
        texture::{Texture, TextureData, TextureUnit},
    },
    geometry::parametric::{Cone, Cylinder, Sphere},
    material,
};

struct Example {
    renderer: Renderer,
    scene: Rc<Node>,
    camera: Rc<RefCell<Camera>>,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        let renderer = Renderer::initialize(context, Default::default(), None);
        let scene = Node::new_group();

        let camera = Camera::new_perspective(Default::default());
        {
            let camera = Node::new_camera(Rc::clone(&camera));
            camera.rotate_x(-Angle::from_degrees(20.0), Default::default());
            camera.set_position(&glm::vec3(0.0, 1.0, 4.0));
            scene.add_child(&camera);
        }

        let material = Rc::new(material::texture::create(
            context,
            Texture::initialize(
                context,
                TextureData::load_from_source("images/grid.png").await?,
                Default::default(),
            )?,
            TextureUnit::from(0),
            Default::default(),
        )?);
        {
            let geometry = Rc::new(Geometry::from_with_context(context, Sphere::default())?);
            let mesh = Node::new_mesh(Mesh::initialize(context, geometry, Rc::clone(&material))?);
            mesh.apply_matrix(&matrix::translation(-3.0, -0.5, 0.0), Default::default());
            scene.add_child(&mesh);
        }
        {
            let geometry = Rc::new(Geometry::from_with_context(
                context,
                Cone {
                    radius: 1.0,
                    height: 2.0,
                    ..Default::default()
                },
            )?);
            let mesh = Node::new_mesh(Mesh::initialize(context, geometry, Rc::clone(&material))?);
            mesh.apply_matrix(&matrix::translation(0.0, -0.5, 0.0), Default::default());
            scene.add_child(&mesh);
        }
        {
            let geometry = Rc::new(Geometry::from_with_context(
                context,
                Cylinder {
                    radius: 0.8,
                    height: 2.0,
                    ..Default::default()
                },
            )?);
            let mesh = Node::new_mesh(Mesh::initialize(context, geometry, Rc::clone(&material))?);
            mesh.apply_matrix(&matrix::translation(3.0, -0.5, 0.0), Default::default());
            scene.add_child(&mesh);
        }

        Ok(Box::new(Example {
            renderer,
            scene,
            camera,
        }))
    }
}

impl Application for Example {
    fn update(&mut self, _key_state: &KeyState) {}

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
