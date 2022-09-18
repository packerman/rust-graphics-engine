use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::core::{
    application::{self, Application, AsyncCreator},
    camera::Camera,
    convert::FromWithContext,
    geometry::{Geometry, Rectangle},
    input::KeyState,
    material::{Material, MaterialSettings},
    mesh::Mesh,
    node::Node,
    renderer::{Renderer, RendererOptions},
};

struct Example {
    renderer: Renderer,
    scene: Rc<Node>,
    camera: Rc<RefCell<Camera>>,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Self> {
        let renderer = Renderer::new_initialized(context, RendererOptions::default());
        let scene = Node::new_group();

        let camera = Rc::new(RefCell::new(Camera::default()));
        {
            let camera = Node::new_camera(Rc::clone(&camera));
            camera.set_position(&glm::vec3(0.0, 0.0, 1.5));
            scene.add_child(&camera);
        }
        let material = Rc::new(Material::from_with_context(
            context,
            MaterialSettings {
                vertex_shader: include_str!("vertex.glsl"),
                fragment_shader: include_str!("fragment.glsl"),
                uniforms: vec![],
                render_settings: vec![],
                draw_style: WebGl2RenderingContext::TRIANGLES,
            },
        )?);
        {
            let geometry = Geometry::from_with_context(
                context,
                Rectangle {
                    width: 1.5,
                    height: 1.5,
                },
            )?;

            let mesh = Node::new_mesh(Box::new(Mesh::new(context, geometry, material)?));
            scene.add_child(&mesh);
        }

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
