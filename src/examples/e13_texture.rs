use std::rc::Rc;

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::{
    api::geometry::Geometry,
    base::{
        application::{self, Application, AsyncCreator},
        convert::FromWithContext,
        input::KeyState,
        util::shared_ref::SharedRef,
    },
    core::{
        camera::{Camera, Perspective},
        mesh::Mesh,
        node::Node,
        scene::Scene,
        texture::{Texture, TextureUnit},
    },
    geometry::Rectangle,
    legacy::renderer::Renderer,
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
        let renderer = Renderer::initialize(context, Default::default(), None);
        let mut scene = Scene::new_empty();

        let camera = Camera::new(Perspective::default());
        let camera_node = Node::new_with_camera(Rc::clone(&camera));
        camera_node
            .borrow_mut()
            .set_position(&glm::vec3(0.0, 0.0, 1.0));
        scene.add_root_node(camera_node);

        let geometry = Geometry::from_with_context(context, Rectangle::default())?;
        let material = material::texture::create(
            context,
            Texture::fetch(context, "images/grid.png").await?,
            TextureUnit(0),
            Default::default(),
        )?;
        let mesh = Node::new_with_mesh(Mesh::initialize(context, &geometry, material)?);
        scene.add_root_node(mesh);
        Ok(Box::new(Example {
            renderer,
            scene,
            camera,
        }))
    }
}

impl Application for Example {
    fn name(&self) -> &str {
        "Texture"
    }

    fn update(&mut self, _key_state: &KeyState) {}

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
