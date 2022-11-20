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
        light::{shadow::Shadow, Light},
        math::angle::Angle,
        mesh::Mesh,
        node::Node,
        renderer::{self, Renderer, RendererOptions},
        texture::{Texture, TextureData, TextureUnit},
        uniform::data::Sampler2D,
    },
    extras::light_helpers::DirectionalLightHelper,
    geometry::{parametric::Sphere, Rectangle},
    material::{self, phong::PhongMaterial},
};

struct Example {
    rig: Rc<Node>,
    renderer: Renderer,
    scene: Rc<Node>,
    camera: Rc<RefCell<Camera>>,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        let scene = Node::new_group();

        let directional_light = Node::new_light(Light::directional(
            Color::new_rgb(1.0, 1.0, 1.0),
            glm::vec3(-1.0, -1.0, 0.0),
        ));
        directional_light.set_position(&glm::vec3(2.0, 4.0, 0.0));
        scene.add_child(&directional_light);
        let directional_helper = Node::new_mesh(
            DirectionalLightHelper::default()
                .create_mesh(context, &directional_light.as_light().unwrap().borrow())?,
        );
        directional_light.add_child(&directional_helper);

        let resolution = renderer::get_canvas_size(context);

        let shadow = Shadow::initialize(
            context,
            Rc::clone(&directional_light),
            resolution,
            TextureUnit::from(15),
            Default::default(),
        )?;

        let renderer = Renderer::initialize(
            context,
            RendererOptions {
                clear_color: Color::new_rgb(0.2, 0.2, 0.2),
                ..Default::default()
            },
            shadow.into(),
        );

        let camera = Node::new_camera(Camera::new_perspective(Default::default()));
        scene.add_child(&camera);

        let rig = Node::new_movement_rig(Default::default());
        {
            rig.add_child(&camera);
            rig.set_position(&glm::vec3(0.0, 2.0, 5.0));
            scene.add_child(&rig);
        }

        let ambient_color = Color::new_rgb(0.2, 0.2, 0.2);

        let sphere_geometry = Rc::new(Geometry::from_with_context(context, Sphere::default())?);
        let phong_material = material::phong::create(
            context,
            PhongMaterial {
                texture: Sampler2D::new(
                    Texture::initialize(
                        context,
                        TextureData::load_from_source("images/grid.png").await?,
                        Default::default(),
                    )?,
                    TextureUnit::from(0),
                )
                .into(),
                ambient: ambient_color,
                shadow: renderer.shadow(),
                ..Default::default()
            },
        )?;

        let sphere1 = Node::new_mesh(Mesh::initialize(
            context,
            Rc::clone(&sphere_geometry),
            Rc::clone(&phong_material),
        )?);
        {
            sphere1.set_position(&glm::vec3(-2.0, 1.0, 0.0));
            scene.add_child(&sphere1);
        }

        let sphere2 = Node::new_mesh(Mesh::initialize(
            context,
            Rc::clone(&sphere_geometry),
            Rc::clone(&phong_material),
        )?);
        {
            sphere2.set_position(&glm::vec3(1.0, 2.2, -0.5));
            scene.add_child(&sphere2);
        }

        let floor = Node::new_mesh(Mesh::initialize(
            context,
            Rc::new(Geometry::from_with_context(
                context,
                Rectangle {
                    width: 20.0,
                    height: 20.0,
                    ..Default::default()
                },
            )?),
            Rc::clone(&phong_material),
        )?);
        floor.rotate_x(-Angle::RIGHT, Default::default());
        scene.add_child(&floor);

        Ok(Box::new(Example {
            rig,
            renderer,
            scene,
            camera: Rc::clone(camera.as_camera().unwrap()),
        }))
    }
}

impl Application for Example {
    fn update(&mut self, key_state: &KeyState) {
        self.rig.update_key_state(key_state);
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
