use std::rc::Rc;

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::{
    api::geometry::Geometry,
    base::{
        application::{self, Application, AsyncCreator},
        color,
        convert::FromWithContext,
        input::KeyState,
        util::shared_ref::SharedRef,
        web,
    },
    classic::{light::Light, renderer::Renderer, texture::Sampler2D},
    core::{
        camera::{Camera, Perspective},
        mesh::Mesh,
        node::Node,
        scene::Scene,
        texture::{Texture, TextureUnit},
    },
    extras::light_helpers::{DirectionalLightHelper, PointLightHelper},
    geometry::parametric::Sphere,
    material::{self, flat::FlatMaterial, lambert::LambertMaterial, phong::PhongMaterial},
};

struct Example {
    renderer: Renderer,
    scene: Scene,
    camera: SharedRef<Camera>,
    point: Rc<Node>,
    directional: Rc<Node>,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        let mut renderer = Renderer::initialize(context, Default::default(), None);
        let scene = Scene::new_empty();

        let camera = Camera::new(Perspective::default());
        {
            let camera = Node::new_with_camera(Rc::clone(&camera));
            camera.borrow_mut().set_position(&glm::vec3(0.0, 0.0, 6.0));
            scene.add_node(camera);
        }

        let directional = Node::new_light(Light::directional(
            color::rgb(0.8, 0.8, 0.8),
            glm::vec3(-1.0, -1.0, -2.0),
        ));
        scene.add_child(&directional);
        let point = Node::new_light(Light::point(
            color::rgb(0.9, 0.0, 0.0),
            glm::vec3(1.0, 1.0, 0.8),
        ));
        scene.add_child(&point);

        {
            let direct_helper = Node::new_with_mesh(
                DirectionalLightHelper::default()
                    .create_mesh(context, &directional.as_light().unwrap().borrow())?,
            );
            directional.set_position(&glm::vec3(3.0, 2.0, 0.0));
            directional.add_child(&direct_helper);
            let point_helper = Node::new_with_mesh(
                PointLightHelper::default()
                    .create_mesh(context, &point.as_light().unwrap().borrow())?,
            );
            point.add_child(&point_helper);
        }

        {
            let sphere1 = Node::new_with_mesh(Mesh::initialize(
                context,
                &Geometry::from_with_context(context, Sphere::default())?,
                material::flat::create(
                    context,
                    FlatMaterial {
                        ambient: color::rgb(0.1, 0.1, 0.1),
                        diffuse: color::rgb(0.6, 0.2, 0.2),
                        ..Default::default()
                    },
                )?,
            )?);
            sphere1
                .borrow_mut()
                .set_position(&glm::vec3(-2.2, 0.0, 0.0));
            scene.add_node(sphere1);
        }
        {
            let sphere2 = Node::new_with_mesh(Mesh::initialize(
                context,
                &Geometry::from_with_context(context, Sphere::default())?,
                material::lambert::create(
                    context,
                    LambertMaterial {
                        ambient: color::rgb(0.1, 0.1, 0.1),
                        texture: Sampler2D::new(
                            Texture::fetch(context, "images/grid.png").await?,
                            TextureUnit(0),
                        )
                        .into(),
                        ..Default::default()
                    },
                )?,
            )?);
            sphere2.borrow_mut().set_position(&glm::vec3(0.0, 0.0, 0.0));
            scene.add_node(sphere2);
        }
        {
            let sphere3 = Node::new_with_mesh(Mesh::initialize(
                context,
                &Geometry::from_with_context(context, Sphere::default())?,
                material::phong::create(
                    context,
                    PhongMaterial {
                        ambient: color::rgb(0.1, 0.1, 0.1),
                        diffuse: color::rgb(0.5, 0.5, 1.0),
                        ..Default::default()
                    },
                )?,
            )?);
            sphere3.borrow_mut().set_position(&glm::vec3(2.2, 0.0, 0.0));
            scene.add_node(sphere3);
        }
        Ok(Box::new(Example {
            renderer,
            scene,
            camera,
            point,
            directional,
        }))
    }
}

impl Application for Example {
    fn name(&self) -> &str {
        "Lights"
    }

    fn update(&mut self, _key_state: &KeyState) {
        let time = (web::now().unwrap() / 1000.0) as f32;
        self.directional
            .set_direction(&glm::vec3(-1.0, (0.7 * time).sin(), -2.0));
        self.point.set_position(&glm::vec3(1.0, time.sin(), 0.8));
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
