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
    },
    classic::renderer::Renderer,
    core::{
        camera::{Camera, Perspective},
        mesh::Mesh,
        node::Node,
        scene::Scene,
        texture::{Texture, TextureUnit},
    },
    geometry::parametric::{Cone, Cylinder, Sphere},
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

        let camera = Camera::new(Perspective::default());
        {
            let camera = Node::new_with_camera(Rc::clone(&camera));
            camera.borrow_mut().rotate_x(-Angle::from_degrees(20.0));
            camera.borrow_mut().set_position(&glm::vec3(0.0, 1.0, 4.0));
            scene.add_node(camera);
        }

        let material = material::texture::create(
            context,
            Texture::fetch(context, "images/grid.png").await?,
            TextureUnit(0),
            Default::default(),
        )?;
        {
            let geometry = Geometry::from_with_context(context, Sphere::default())?;
            let mesh =
                Node::new_with_mesh(Mesh::initialize(context, &geometry, Rc::clone(&material))?);
            mesh.borrow_mut()
                .apply_transform(&matrix::translation(-3.0, -0.5, 0.0));
            scene.add_node(mesh);
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
            let mesh =
                Node::new_with_mesh(Mesh::initialize(context, &geometry, Rc::clone(&material))?);
            mesh.borrow_mut()
                .apply_transform(&matrix::translation(0.0, -0.5, 0.0));
            scene.add_node(mesh);
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
            let mesh = Node::new_with_mesh(Mesh::initialize(context, &geometry, material)?);
            mesh.borrow_mut()
                .apply_transform(&matrix::translation(3.0, -0.5, 0.0));
            scene.add_node(mesh);
        }

        Ok(Box::new(Example {
            renderer,
            scene,
            camera,
        }))
    }
}

impl Application for Example {
    fn name(&self) -> &str {
        "More textures"
    }

    fn update(&mut self, _key_state: &KeyState) {}

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
