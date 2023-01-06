use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::{
    api::geometry::Geometry,
    base::{
        application::{self, Application, AsyncCreator},
        convert::FromWithContext,
        input::KeyState,
        math::{angle::Angle, matrix},
        util::shared_ref,
    },
    core::{
        camera::{Camera, Perspective},
        mesh::Mesh,
        node::Node,
        scene::Scene,
        texture::{Texture, TextureUnit},
    },
    geometry::parametric::{Cone, Cylinder, Sphere},
    legacy::renderer::Renderer,
    material,
};

struct Example {
    renderer: Renderer,
    scene: Scene,
    camera: Rc<RefCell<Camera>>,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        let renderer = Renderer::initialize(context, Default::default(), None);
        let mut scene = Scene::new_empty();

        let camera = shared_ref::strong(Camera::from(Perspective::default()));
        {
            let camera = Node::new_with_camera(Rc::clone(&camera));
            camera.borrow_mut().rotate_x(-Angle::from_degrees(20.0));
            camera.borrow_mut().set_position(&glm::vec3(0.0, 1.0, 4.0));
            scene.add_root_node(camera);
        }

        let material = Rc::new(material::texture::create(
            context,
            Rc::new(Texture::fetch(context, "images/grid.png").await?),
            TextureUnit(0),
            Default::default(),
        )?);
        {
            let geometry = Rc::new(Geometry::from_with_context(context, Sphere::default())?);
            let mesh = Node::new_with_mesh(Rc::new(
                geometry.create_mesh(context, Rc::clone(&material))?,
            ));
            mesh.borrow_mut()
                .apply_transform(&matrix::translation(-3.0, -0.5, 0.0));
            scene.add_root_node(mesh);
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
            let mesh = Node::new_with_mesh(Rc::new(
                geometry.create_mesh(context, Rc::clone(&material))?,
            ));
            mesh.borrow_mut()
                .apply_transform(&matrix::translation(0.0, -0.5, 0.0));
            scene.add_root_node(mesh);
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
            let mesh = Node::new_with_mesh(Rc::new(
                geometry.create_mesh(context, Rc::clone(&material))?,
            ));
            mesh.borrow_mut()
                .apply_transform(&matrix::translation(3.0, -0.5, 0.0));
            scene.add_root_node(mesh);
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
