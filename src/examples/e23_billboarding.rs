use std::rc::Rc;

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
        math::{angle::Angle, matrix},
        mesh::Mesh,
        node::Node,
        renderer::{Renderer, RendererOptions},
        texture::{Texture, TextureData, TextureUnit},
    },
    extras::text_texture::TextTexture,
    geometry::{BoxGeometry, Rectangle},
    material,
};

struct Example {
    renderer: Renderer,
    scene: Rc<Node>,
    rig: Rc<Node>,
    camera: Rc<Node>,
    label: Rc<Node>,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        let renderer = Renderer::new(
            context,
            RendererOptions {
                clear_color: Color::dark_slate_gray(),
                ..Default::default()
            },
        );
        let scene = Node::new_group();

        let rig = Node::new_movement_rig(Default::default());
        let camera = Node::new_camera(Camera::new_perspective(Default::default()));
        {
            rig.set_position(&glm::vec3(0.0, 1.0, 5.0));
            rig.add_child(&camera);
            scene.add_child(&rig);
        }
        let label = self::create_label(context)?;
        {
            label.set_position(&glm::vec3(0.0, 1.0, 0.0));
            scene.add_child(&label);
        }
        {
            let crate_mesh = create_crate_mesh(context).await?;
            scene.add_child(&crate_mesh);
        }
        Ok(Box::new(Example {
            renderer,
            scene,
            rig,
            camera,
            label,
        }))
    }
}

fn create_label(context: &WebGl2RenderingContext) -> Result<Rc<Node>> {
    let texture = Texture::initialize(
        context,
        TextureData::try_from(TextTexture {
            text: "This is a Crate.",
            width: 320,
            height: 160,
            border_width: 4.0,
            font: "bold 40px arial",
            font_style: "blue",
            ..Default::default()
        })?,
        Default::default(),
    )?;
    let material =
        material::texture::create(context, texture, TextureUnit::from(0), Default::default())?;
    let mut geometry = Geometry::from_with_context(
        context,
        Rectangle {
            width: 1.0,
            height: 0.5,
            ..Default::default()
        },
    )?;
    geometry.apply_matrix_default(context, &matrix::rotation_y(Angle::STRAIGHT))?;
    let label = Mesh::initialize(context, Rc::new(geometry), material)?;
    Ok(Node::new_mesh(label))
}

async fn create_crate_mesh(context: &WebGl2RenderingContext) -> Result<Rc<Node>> {
    let geometry = Rc::new(Geometry::from_with_context(
        context,
        BoxGeometry::default(),
    )?);
    let material = material::texture::create(
        context,
        Texture::initialize(
            context,
            TextureData::load_from_source("images/crate.png").await?,
            Default::default(),
        )?,
        TextureUnit::from(1),
        Default::default(),
    )?;
    let mesh = Mesh::initialize(context, geometry, material)?;
    Ok(Node::new_mesh(mesh))
}

impl Application for Example {
    fn update(&mut self, key_state: &KeyState) {
        self.rig.update_key_state(key_state);
        self.label.look_at(&self.camera.world_position());
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer
            .render(context, &self.scene, self.camera.camera().unwrap())
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
