use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::{
    core::{
        application::{self, Application, AsyncCreator},
        camera::Camera,
        color::Color,
        convert::FromWithContext,
        geometry::Geometry,
        input::KeyState,
        mesh::Mesh,
        node::Node,
        renderer::Renderer,
        texture::{Texture, TextureData, TextureUnit},
        uniform::data::Sampler2D,
    },
    geometry::parametric::Sphere,
    light::{Light, LightType},
    material::{self, flat::FlatMaterial, lambert::LambertMaterial, phong::PhongMaterial},
};

struct Example {
    renderer: Renderer,
    scene: Rc<Node>,
    camera: Rc<RefCell<Camera>>,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        let renderer = Renderer::new(context, Default::default());
        let scene = Node::new_group();

        let camera = Camera::new_perspective(Default::default());
        {
            let camera = Node::new_camera(Rc::clone(&camera));
            camera.set_position(&glm::vec3(0.0, 0.0, 6.0));
            scene.add_child(&camera);
        }
        // let directional = Node::new_light(Light {
        //     light_type: LightType::directional(glm::vec3(-1.0, -1.0, -2.0)).into(),
        //     color: Color::new_rgb(0.8, 0.8, 0.8),
        //     ..Default::default()
        // });
        // scene.add_child(&directional);
        let point = Node::new_light(Light {
            light_type: LightType::point(glm::vec3(1.0, 1.0, 0.8)).into(),
            color: Color::new_rgb(0.9, 0.0, 0.0),
            ..Default::default()
        });
        scene.add_child(&point);

        {
            let geometry = Rc::new(Geometry::from_with_context(context, Sphere::default())?);
            let material = material::flat::create(
                context,
                FlatMaterial {
                    ambient: Color::new_rgb(0.1, 0.1, 0.1),
                    diffuse: Color::new_rgb(0.6, 0.2, 0.2),
                    ..Default::default()
                },
            )?;
            let mesh = Node::new_mesh(Mesh::initialize(context, geometry, material)?);
            mesh.set_position(&glm::vec3(-2.2, 0.0, 0.0));
            scene.add_child(&mesh);
        }
        {
            let geometry = Rc::new(Geometry::from_with_context(context, Sphere::default())?);
            let material = material::lambert::create(
                context,
                LambertMaterial {
                    ambient: Color::new_rgb(0.1, 0.1, 0.1),
                    texture: Sampler2D::new(
                        Texture::initialize(
                            context,
                            TextureData::load_from_source("images/grid.png").await?,
                            Default::default(),
                        )?,
                        TextureUnit::from(0),
                    )
                    .into(),
                    ..Default::default()
                },
            )?;
            let mesh = Node::new_mesh(Mesh::initialize(context, geometry, material)?);
            mesh.set_position(&glm::vec3(0.0, 0.0, 0.0));
            scene.add_child(&mesh);
        }
        {
            let geometry = Rc::new(Geometry::from_with_context(context, Sphere::default())?);
            let material = material::phong::create(
                context,
                PhongMaterial {
                    ambient: Color::new_rgb(0.1, 0.1, 0.1),
                    diffuse: Color::new_rgb(0.5, 0.5, 1.0),
                    ..Default::default()
                },
            )?;
            let mesh = Node::new_mesh(Mesh::initialize(context, geometry, material)?);
            mesh.set_position(&glm::vec3(2.2, 0.0, 0.0));
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
