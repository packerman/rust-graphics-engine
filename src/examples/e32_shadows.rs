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
        math::angle::Angle,
        util::shared_ref::SharedRef,
    },
    classic::{
        light::{Light, Lights},
        renderer::{self, Renderer, RendererOptions},
        shadow::Shadow,
        texture::Sampler2D,
    },
    core::{
        camera::{Camera, Perspective},
        mesh::Mesh,
        node::Node,
        scene::Scene,
        texture::{Texture, TextureUnit},
    },
    extras::{camera_controller::CameraController, light_helpers::DirectionalLightHelper},
    geometry::{parametric::Sphere, rectangle::Rectangle},
    material::{self, phong::PhongMaterial},
};

struct Example {
    controller: CameraController,
    renderer: Renderer,
    scene: Scene,
    camera: SharedRef<Camera>,
    lights: Lights,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        let mut scene = Scene::new_empty();
        let mut lights = Lights::new();

        let directional_light = lights.create_node(Light::directional(
            color::rgb(1.0, 1.0, 1.0),
            glm::vec3(-1.0, -1.0, 0.0),
        ));
        directional_light.set_position(&glm::vec3(2.0, 4.0, 0.0));
        directional_light.add_to_scene(&mut scene);
        let directional_helper = Node::new_with_mesh(
            DirectionalLightHelper::default()
                .create_mesh(context, &*directional_light.light().borrow())?,
        );
        directional_light.add_child(directional_helper);

        let resolution = renderer::get_canvas_resolution(context);

        let shadow = Shadow::initialize(
            context,
            Rc::clone(&directional_light),
            resolution,
            TextureUnit(15),
            Default::default(),
        )?;

        let renderer = Renderer::initialize(
            context,
            RendererOptions {
                clear_color: color::rgb(0.2, 0.2, 0.2),
                ..Default::default()
            },
            shadow.into(),
        );

        let camera = Node::new_with_camera(Camera::new(Perspective::default()));
        {
            camera.borrow_mut().set_position(&glm::vec3(0.0, 2.0, 5.0));
            scene.add_node(Rc::clone(&camera));
        }

        let controller = CameraController::make_for_node(Rc::clone(&camera));

        let ambient_color = color::rgb(0.2, 0.2, 0.2);

        let sphere_geometry = Geometry::from_with_context(context, Sphere::default())?;
        let phong_material = material::phong::create(
            context,
            PhongMaterial {
                texture: Sampler2D::new(
                    Texture::fetch(context, "images/grid.png").await?,
                    TextureUnit(0),
                )
                .into(),
                ambient: ambient_color,
                // TODO delete if needed
                // shadow: renderer.shadow(),
                use_shadow: true,
                ..Default::default()
            },
        )?;

        let sphere1 = Node::new_with_mesh(Mesh::initialize(
            context,
            &sphere_geometry,
            Rc::clone(&phong_material),
        )?);
        {
            sphere1
                .borrow_mut()
                .set_position(&glm::vec3(-2.0, 1.0, 0.0));
            scene.add_node(sphere1);
        }

        let sphere2 = Node::new_with_mesh(Mesh::initialize(
            context,
            &sphere_geometry,
            Rc::clone(&phong_material),
        )?);
        {
            sphere2
                .borrow_mut()
                .set_position(&glm::vec3(1.0, 2.2, -0.5));
            scene.add_node(sphere2);
        }

        let floor = Node::new_with_mesh(Mesh::initialize(
            context,
            &Geometry::from_with_context(
                context,
                Rectangle {
                    width: 20.0,
                    height: 20.0,
                    ..Default::default()
                },
            )?,
            Rc::clone(&phong_material),
        )?);
        floor.borrow_mut().rotate_x(-Angle::RIGHT);
        scene.add_node(floor);

        Ok(Box::new(Example {
            camera: controller.camera().expect("Camera is present."),
            controller,
            renderer,
            scene,
            lights,
        }))
    }
}

impl Application for Example {
    fn name(&self) -> &str {
        "Shadows"
    }

    fn update(&mut self, key_state: &KeyState) {
        self.controller.update(key_state);
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer
            .render_with_lights(context, &self.scene, &self.camera, &self.lights);
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
