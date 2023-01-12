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
    core::{
        camera::{Camera, Perspective},
        image::Image,
        mesh::Mesh,
        node::Node,
        scene::Scene,
        texture::{Texture, TextureUnit},
    },
    extras::{grid_helper::GridHelper, text_texture::TextTexture},
    geometry::BoxGeometry,
    legacy::renderer::{Renderer, RendererOptions},
    material,
};

struct Example {
    renderer: Renderer,
    scene: Scene,
    camera: SharedRef<Camera>,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        let renderer = Renderer::initialize(
            context,
            RendererOptions {
                clear_color: color::black(),
                ..Default::default()
            },
            None,
        );
        let mut scene = Scene::new_empty();

        let camera = Camera::new(Perspective::default());
        {
            let camera = Node::new_with_camera(Rc::clone(&camera));
            camera.borrow_mut().rotate_y(Angle::from_degrees(-45.0));
            camera.borrow_mut().rotate_x(Angle::from_degrees(-30.0));
            camera
                .borrow_mut()
                .set_position(&glm::vec3(-1.5, 1.5, 1.25));
            scene.add_node(camera);
        }
        {
            let grid = <Rc<Mesh>>::from_with_context(
                context,
                GridHelper {
                    grid_color: color::white(),
                    center_color: color::yellow(),
                    ..Default::default()
                },
            )?;
            let grid = Node::new_with_mesh(grid);
            grid.borrow_mut().rotate_x(-Angle::RIGHT);
            scene.add_node(grid);
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
            Texture::initialize(
                context,
                Default::default(),
                Rc::new(Image::try_from(TextTexture {
                    text: "Hello, World!",
                    font: "bold 36px sans-serif",
                    font_style: "blue",
                    ..Default::default()
                })?),
            )?,
            TextureUnit(0),
            Default::default(),
        )?;
        let mesh = Node::new_with_mesh(Mesh::initialize(context, &geometry, material)?);
        mesh.borrow_mut().set_position(&glm::vec3(0.0, 0.5, 0.0));
        scene.add_node(mesh);
        Ok(Box::new(Example {
            renderer,
            scene,
            camera,
        }))
    }
}

impl Application for Example {
    fn name(&self) -> &str {
        "Text texture"
    }

    fn update(&mut self, _key_state: &KeyState) {}

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
