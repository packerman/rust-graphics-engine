use std::rc::Rc;

use anyhow::Result;
use async_trait::async_trait;
use glm::Vec3;
use web_sys::WebGl2RenderingContext;

use crate::{
    api::geometry::Geometry,
    base::{
        application::{self, Application, AsyncCreator},
        convert::FromWithContext,
        input::KeyState,
        util::shared_ref::{self, SharedRef},
    },
    core::{
        camera::{Camera, Perspective},
        material::{GenericMaterial, Material, Source},
        mesh::Mesh,
        node::Node,
        program::{Program, UpdateProgramUniforms},
        scene::Scene,
    },
    geometry::Rectangle,
    legacy::renderer::{Renderer, RendererOptions},
};

struct Example {
    renderer: Renderer,
    scene: Scene,
    camera: SharedRef<Camera>,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        let renderer = Renderer::initialize(context, RendererOptions::default(), None);
        let mut scene = Scene::new_empty();

        let camera = Camera::new(Perspective::default());
        {
            let camera = Node::new_with_camera(Rc::clone(&camera));
            camera.borrow_mut().set_position(&glm::vec3(0.0, 0.0, 1.5));
            scene.add_root_node(camera);
        }
        let x = 0.6;
        let y = 0.4;
        scene.add_root_node(rectangle_mesh(
            context,
            clouds(context)?,
            glm::vec3(-x, y, 0.0),
        )?);
        scene.add_root_node(rectangle_mesh(
            context,
            lava(context)?,
            glm::vec3(x, y, 0.0),
        )?);
        scene.add_root_node(rectangle_mesh(
            context,
            marble(context)?,
            glm::vec3(-x, -y, 0.0),
        )?);
        scene.add_root_node(rectangle_mesh(
            context,
            wood(context)?,
            glm::vec3(x, -y, 0.0),
        )?);

        Ok(Box::new(Example {
            renderer,
            scene,
            camera,
        }))
    }
}

fn rectangle_mesh(
    context: &WebGl2RenderingContext,
    material: Rc<Material>,
    position: Vec3,
) -> Result<SharedRef<Node>> {
    let geometry = Geometry::from_with_context(
        context,
        Rectangle {
            width: 0.7,
            height: 0.7,
            ..Default::default()
        },
    )?;
    let node = Node::new_with_mesh(Mesh::initialize(context, &geometry, material)?);
    node.borrow_mut().set_position(&position);
    Ok(node)
}

fn clouds(context: &WebGl2RenderingContext) -> Result<Rc<Material>> {
    fractal_material(context, include_str!("clouds.glsl"))
}

fn lava(context: &WebGl2RenderingContext) -> Result<Rc<Material>> {
    fractal_material(context, include_str!("lava.glsl"))
}

fn marble(context: &WebGl2RenderingContext) -> Result<Rc<Material>> {
    fractal_material(context, include_str!("marble.glsl"))
}

fn wood(context: &WebGl2RenderingContext) -> Result<Rc<Material>> {
    fractal_material(context, include_str!("wood.glsl"))
}

fn fractal_material(
    context: &WebGl2RenderingContext,
    source: &'static str,
) -> Result<Rc<Material>> {
    <Rc<Material>>::from_with_context(
        context,
        shared_ref::strong(FractalMaterial {
            main_file: source.into(),
        }),
    )
}

impl Application for Example {
    fn name(&self) -> &str {
        "Procedural texture"
    }

    fn update(&mut self, _key_state: &KeyState) {}

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}

#[derive(Debug, Clone)]
struct FractalMaterial<'a> {
    main_file: Source<'a>,
}

impl GenericMaterial for FractalMaterial<'_> {
    fn vertex_shader(&self) -> Source<'_> {
        include_str!("vertex.glsl").into()
    }

    fn fragment_shader(&self) -> Source<'_> {
        format!("{}\n\n{}\n", include_str!("fragment.glsl"), self.main_file).into()
    }
}

impl UpdateProgramUniforms for FractalMaterial<'_> {
    fn update_program_uniforms(&self, _context: &WebGl2RenderingContext, _programm: &Program) {}
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
