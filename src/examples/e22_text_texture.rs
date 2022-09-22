use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::core::{
    application::{self, Application, AsyncCreator},
    camera::Camera,
    color::Color,
    convert::FromWithContext,
    extras::{text_texture::TextTexture, GridHelper},
    geometry::{BoxGeometry, Geometry},
    input::KeyState,
    material,
    matrix::Angle,
    mesh::Mesh,
    node::Node,
    renderer::{Renderer, RendererOptions},
    texture::{Texture, TextureData, TextureUnit},
};

struct Example {
    renderer: Renderer,
    scene: Rc<Node>,
    camera: Rc<RefCell<Camera>>,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Self> {
        let renderer = Renderer::new(
            context,
            RendererOptions {
                clear_color: Color::black(),
                ..Default::default()
            },
        );
        let scene = Node::new_group();

        let camera = Rc::new(RefCell::new(Camera::default()));
        {
            let camera = Node::new_camera(Rc::clone(&camera));
            camera.rotate_y(Angle::from_degrees(-45.0), Default::default());
            camera.rotate_x(Angle::from_degrees(-30.0), Default::default());
            camera.set_position(&glm::vec3(-1.5, 1.5, 1.25));
            scene.add_child(&camera);
        }
        {
            let grid = Mesh::from_with_context(
                context,
                GridHelper {
                    grid_color: Color::white(),
                    center_color: Color::yellow(),
                    ..Default::default()
                },
            )?;
            let grid = Node::new_mesh(Box::new(grid));
            grid.rotate_x(-Angle::RIGHT, Default::default());
            scene.add_child(&grid);
        }

        let geometry = Geometry::from_with_context(
            context,
            BoxGeometry {
                width: 1.25,
                height: 1.25,
                depth: 1.25,
            },
        )?;
        let material = material::texture::create(
            context,
            Texture::new(
                context,
                TextureData::try_from(TextTexture {
                    text: "Hello, World!",
                    font: "bold 36px sans-serif",
                    font_style: "blue",
                    ..Default::default()
                })?,
                Default::default(),
            )?,
            TextureUnit::from(0),
            Default::default(),
        )?;
        let mesh = Node::new_mesh(Box::new(Mesh::new(context, geometry, Rc::new(material))?));
        mesh.set_position(&glm::vec3(0.0, 0.5, 0.0));
        scene.add_child(&mesh);
        Ok(Example {
            renderer,
            scene,
            camera,
        })
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
