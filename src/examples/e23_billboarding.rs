use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::core::{
    application::{self, Application, AsyncCreator},
    camera::Camera,
    color::Color,
    convert::FromWithContext,
    extras::text_texture::TextTexture,
    geometry::{BoxGeometry, Geometry, Rectangle},
    input::KeyState,
    material,
    matrix::{self, Angle},
    mesh::Mesh,
    node::Node,
    renderer::{Renderer, RendererOptions},
    texture::{Texture, TextureData, TextureUnit},
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
    async fn create(context: &WebGl2RenderingContext) -> Result<Self> {
        let renderer = Renderer::new(
            context,
            RendererOptions {
                clear_color: Color::dark_slate_gray(),
                ..Default::default()
            },
        );
        let scene = Node::new_group();

        let rig = Node::new_movement_rig(Default::default());
        let camera = Node::new_camera(Rc::new(RefCell::new(Camera::default())));
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
        Ok(Example {
            renderer,
            scene,
            rig,
            camera,
            label,
        })
    }
}

fn create_label(context: &WebGl2RenderingContext) -> Result<Rc<Node>> {
    let texture = Texture::new(
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
    let material = Rc::new(material::texture::create(
        context,
        texture,
        TextureUnit::from(0),
        Default::default(),
    )?);
    let mut geometry = Geometry::from_with_context(
        context,
        Rectangle {
            width: 1.0,
            height: 0.5,
        },
    )?;
    geometry.apply_matrix_mut(
        context,
        &matrix::rotation_y(Angle::STRAIGHT),
        "vertexPosition",
    )?;
    let label = Box::new(Mesh::new(context, geometry, material)?);
    Ok(Node::new_mesh(label))
}

async fn create_crate_mesh(context: &WebGl2RenderingContext) -> Result<Rc<Node>> {
    let geometry = Geometry::from_with_context(context, BoxGeometry::default())?;
    let material = Rc::new(material::texture::create(
        context,
        Texture::new(
            context,
            TextureData::load_from_source("images/crate.png").await?,
            Default::default(),
        )?,
        TextureUnit::from(1),
        Default::default(),
    )?);
    let mesh = Box::new(Mesh::new(context, geometry, material)?);
    Ok(Node::new_mesh(mesh))
}

impl Application for Example {
    fn update(&mut self, key_state: &KeyState) {
        self.rig.update(key_state);
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
