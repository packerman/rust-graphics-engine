use std::rc::Rc;

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::{
    api::geometry::{Geometry, TypedGeometry},
    base::{
        application::{self, Application, AsyncCreator},
        color,
        convert::FromWithContext,
        input::KeyState,
        math::{angle::Angle, matrix},
        util::shared_ref::SharedRef,
    },
    classic::renderer::{Renderer, RendererOptions},
    core::{
        camera::{Camera, Perspective},
        image::Image,
        mesh::Mesh,
        node::Node,
        scene::Scene,
        texture::{Texture, TextureUnit},
    },
    extras::{camera_controller::CameraController, text_texture::TextTexture},
    geometry::{box_geom::BoxGeometry, rectangle::Rectangle},
    material,
};

struct Example {
    renderer: Renderer,
    scene: Scene,
    controller: CameraController,
    camera: SharedRef<Camera>,
    label: SharedRef<Node>,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        let renderer = Renderer::initialize(
            context,
            RendererOptions {
                clear_color: color::dark_slate_gray(),
                ..Default::default()
            },
            None,
        );
        let mut scene = Scene::new_empty();

        let camera = Camera::new(Perspective::default());
        {
            let camera = Node::new_with_camera(Rc::clone(&camera));
            camera.borrow_mut().set_position(&glm::vec3(0.0, 1.0, 5.0));
            scene.add_node(Rc::clone(&camera));
        }
        let controller =
            CameraController::make_for_camera(&camera).expect("Camera controller is created");
        let label = self::create_label(context)?;
        {
            label.borrow_mut().set_position(&glm::vec3(0.0, 1.0, 0.0));
            scene.add_node(Rc::clone(&label));
        }
        {
            let crate_mesh = create_crate_mesh(context).await?;
            scene.add_node(crate_mesh);
        }
        Ok(Box::new(Example {
            renderer,
            scene,
            controller,
            camera,
            label,
        }))
    }
}

fn create_label(context: &WebGl2RenderingContext) -> Result<SharedRef<Node>> {
    let texture = Texture::initialize(
        context,
        Default::default(),
        Rc::new(Image::try_from(TextTexture {
            text: "This is a Crate.",
            width: 320,
            height: 160,
            border_width: 4.0,
            font: "bold 40px arial",
            font_style: "blue",
            ..Default::default()
        })?),
    )?;
    let material = material::texture::create(context, texture, TextureUnit(0), Default::default())?;
    let mut typed_geometry = TypedGeometry::try_from(Rectangle {
        width: 1.0,
        height: 0.5,
        ..Default::default()
    })?;
    typed_geometry.transform_mut(&matrix::rotation_y(Angle::STRAIGHT));
    let geometry = Geometry::from_with_context(context, typed_geometry)?;
    let label = Mesh::initialize(context, &geometry, material)?;
    Ok(Node::new_with_mesh(label))
}

async fn create_crate_mesh(context: &WebGl2RenderingContext) -> Result<SharedRef<Node>> {
    let geometry = Geometry::from_with_context(context, BoxGeometry::default())?;
    let material = material::texture::create(
        context,
        Texture::fetch(context, "images/crate.png").await?,
        TextureUnit(1),
        Default::default(),
    )?;
    let mesh = Mesh::initialize(context, &geometry, material)?;
    Ok(Node::new_with_mesh(mesh))
}

impl Application for Example {
    fn name(&self) -> &str {
        "Billboarding"
    }

    fn update(&mut self, key_state: &KeyState) {
        self.controller.update(key_state);
        self.label
            .borrow_mut()
            .look_at(&self.camera.borrow().world_position());
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
