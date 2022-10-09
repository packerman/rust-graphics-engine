use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use async_trait::async_trait;
use glm::Vec3;
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
        let renderer = Renderer::new(context, RendererOptions::default());
        let scene = Node::new_group();

        let camera = Camera::new_perspective(Default::default());
        {
            let camera = Node::new_camera(Rc::clone(&camera));
            camera.set_position(&glm::vec3(0.0, 0.0, 1.5));
            scene.add_child(&camera);
        }
        let x = 0.6;
        let y = 0.4;
        scene.add_child(&(rectangle_mesh(context, clouds(context)?, glm::vec3(-x, y, 0.0))?));
        scene.add_child(&(rectangle_mesh(context, lava(context)?, glm::vec3(x, y, 0.0))?));
        scene.add_child(&(rectangle_mesh(context, marble(context)?, glm::vec3(-x, -y, 0.0))?));
        scene.add_child(&(rectangle_mesh(context, wood(context)?, glm::vec3(x, -y, 0.0))?));

        Ok(Example {
            renderer,
            scene,
            camera,
        })
    }
}

fn rectangle_mesh(
    context: &WebGl2RenderingContext,
    material: Material,
    position: Vec3,
) -> Result<Rc<Node>> {
    let geometry = <Box<Geometry>>::from_with_context(
        context,
        Rectangle {
            width: 0.7,
            height: 0.7,
            ..Default::default()
        },
    )?;
    let node = Node::new_mesh(Mesh::initialize(context, geometry, Rc::new(material))?);
    node.set_position(&position);
    Ok(node)
}

fn clouds(context: &WebGl2RenderingContext) -> Result<Material> {
    fractal_material(context, include_str!("clouds.glsl"))
}

fn lava(context: &WebGl2RenderingContext) -> Result<Material> {
    fractal_material(context, include_str!("lava.glsl"))
}

fn marble(context: &WebGl2RenderingContext) -> Result<Material> {
    fractal_material(context, include_str!("marble.glsl"))
}

fn wood(context: &WebGl2RenderingContext) -> Result<Material> {
    fractal_material(context, include_str!("wood.glsl"))
}

fn fractal_material(context: &WebGl2RenderingContext, main_file: &str) -> Result<Material> {
    Material::from_with_context(
        context,
        MaterialSettings {
            vertex_shader: include_str!("vertex.glsl"),
            fragment_shader: &format!("{}\n\n{}\n", include_str!("fragment.glsl"), main_file),
            uniforms: vec![],
            render_settings: vec![],
            draw_style: WebGl2RenderingContext::TRIANGLES,
        },
    )
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
