use std::{f32::consts::TAU, rc::Rc};

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::{
    api::geometry::Geometry,
    base::{
        application::{self, Application, AsyncCreator},
        convert::FromWithContext,
        input::KeyState,
        math::angle::Angle,
        util::shared_ref::{self, SharedRef},
    },
    classic::renderer::{Renderer, RendererOptions},
    core::{
        camera::{Camera, Perspective},
        material::Material,
        mesh::Mesh,
        node::Node,
        scene::Scene,
    },
    geometry::box_geom::BoxGeometry,
    material::basic::{BasicMaterial, SurfaceMaterial},
};

struct Example {
    renderer: Renderer,
    scene: Scene,
    mesh: SharedRef<Node>,
    camera: SharedRef<Camera>,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        let renderer = Renderer::initialize(context, RendererOptions::default(), None);
        let mut scene = Scene::new_empty();

        let camera = Camera::new(Perspective::default());
        let camera_node = Node::new_with_camera(Rc::clone(&camera));
        camera_node
            .borrow_mut()
            .set_position(&glm::vec3(0.0, 0.0, 2.0));
        scene.add_node(camera_node);

        let geometry = Geometry::from_with_context(context, BoxGeometry::default())?;
        let material = <Rc<Material>>::from_with_context(
            context,
            shared_ref::new(SurfaceMaterial {
                basic: BasicMaterial {
                    use_vertex_colors: true,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )?;
        let mesh = Mesh::initialize(context, &geometry, material)?;
        let mesh = Node::new_with_mesh(mesh);
        scene.add_node(Rc::clone(&mesh));

        Ok(Box::new(Example {
            renderer,
            mesh,
            scene,
            camera,
        }))
    }
}

impl Application for Example {
    fn name(&self) -> &str {
        "Spinning cube"
    }

    fn update(&mut self, _key_state: &KeyState) {
        self.mesh
            .borrow_mut()
            .rotate_y(Angle::from_radians(TAU) / 450.0);
        self.mesh
            .borrow_mut()
            .rotate_x(Angle::from_radians(TAU) / 600.0);
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
