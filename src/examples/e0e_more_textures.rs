use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::core::{
    application::{Application, AsyncCreator},
    camera::Camera,
    convert::FromWithContext,
    geometry::{
        parametric::{Cone, Cylinder, Sphere},
        Geometry,
    },
    input::KeyState,
    material,
    matrix::{self, Angle},
    mesh::Mesh,
    node::Node,
    renderer::Renderer,
    texture::{Texture, TextureUnit},
};

pub struct MoreTexturesExample {
    renderer: Renderer,
    scene: Rc<Node>,
    camera: Rc<RefCell<Camera>>,
}

#[async_trait(?Send)]
impl AsyncCreator for MoreTexturesExample {
    async fn create(context: &WebGl2RenderingContext) -> Result<Self> {
        let renderer = Renderer::new_initialized(context, Default::default());
        let scene = Node::new_group();

        let camera = Rc::new(RefCell::new(Camera::default()));
        {
            let camera = Node::new_camera(Rc::clone(&camera));
            camera.rotate_x(-Angle::from_degrees(20.0), Default::default());
            camera.set_position(&glm::vec3(0.0, 1.0, 4.0));
            scene.add_child(&camera);
        }

        let material = Rc::new(material::texture::create(
            context,
            Texture::load_from_source(context, "images/set02/grid.png", Default::default()).await?,
            TextureUnit::from(0),
            Default::default(),
        )?);
        {
            let geometry = Geometry::from_with_context(context, Sphere::default())?;
            let mesh = Node::new_mesh(Box::new(Mesh::new(
                context,
                geometry,
                Rc::clone(&material),
            )?));
            mesh.appply_matrix(&matrix::translation(-3.0, -0.5, 0.0), Default::default());
            scene.add_child(&mesh);
        }
        {
            let geometry = Geometry::from_with_context(
                context,
                Cone {
                    radius: 1.0,
                    height: 2.0,
                    ..Default::default()
                },
            )?;
            let mesh = Node::new_mesh(Box::new(Mesh::new(
                context,
                geometry,
                Rc::clone(&material),
            )?));
            mesh.appply_matrix(&matrix::translation(0.0, -0.5, 0.0), Default::default());
            scene.add_child(&mesh);
        }
        {
            let geometry = Geometry::from_with_context(
                context,
                Cylinder {
                    radius: 0.8,
                    height: 2.0,
                    ..Default::default()
                },
            )?;
            let mesh = Node::new_mesh(Box::new(Mesh::new(
                context,
                geometry,
                Rc::clone(&material),
            )?));
            mesh.appply_matrix(&matrix::translation(3.0, -0.5, 0.0), Default::default());
            scene.add_child(&mesh);
        }

        Ok(MoreTexturesExample {
            renderer,
            scene,
            camera,
        })
    }
}

impl Application for MoreTexturesExample {
    fn update(&mut self, _key_state: &KeyState) {}

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}
