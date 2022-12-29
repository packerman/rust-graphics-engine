use std::{f32::consts::TAU, rc::Rc};

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::{
    api::geometry::Geometry,
    base::{
        application::{self, Application, AsyncCreator},
        input::KeyState,
        math::angle::Angle,
        util::shared_ref::SharedRef,
    },
    core::{camera::Camera, material::Material, node::Node},
    geometry::BoxGeometry,
    legacy::renderer::{Renderer, RendererOptions},
    material::basic::{BasicMaterial, SurfaceMaterial},
};

struct Example {
    renderer: Renderer,
    scene: SharedRef<Node>,
    mesh: SharedRef<Node>,
    camera: SharedRef<Camera>,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        let renderer = Renderer::initialize(context, RendererOptions::default(), None);
        let scene = Node::new_group();

        let camera = Camera::new_perspective(Default::default());
        let camera_node = Node::new_camera(Rc::clone(&camera));
        camera_node.set_position(&glm::vec3(0.0, 0.0, 2.0));
        scene.add_child(&camera_node);

        let geometry = Rc::new(Geometry::from_with_context(
            context,
            BoxGeometry::default(),
        )?);
        let material = Rc::new(Material::from_with_context(
            context,
            SurfaceMaterial {
                basic: BasicMaterial {
                    use_vertex_colors: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        )?);
        let mesh = geometry.create_mesh(context, material)?;
        let mesh = Node::with_mesh(mesh);
        scene.add_child(&mesh);

        Ok(Box::new(Example {
            renderer,
            mesh,
            scene,
            camera,
        }))
    }
}

impl Application for Example {
    fn update(&mut self, _key_state: &KeyState) {
        self.mesh.rotate_y(Angle::from_radians(TAU) / 450.0);
        self.mesh.rotate_x(Angle::from_radians(TAU) / 600.0);
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
