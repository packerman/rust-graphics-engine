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
    classic::renderer::{Renderer, RendererOptions},
    core::{
        camera::{Camera, Orthographic, Perspective},
        mesh::Mesh,
        node::Node,
        scene::Scene,
        texture::{Texture, TextureUnit},
    },
    extras::{camera_controller::CameraController, grid_helper::GridHelper},
    geometry::{box_geom::BoxGeometry, rectangle::Rectangle},
    material,
};

struct Example {
    renderer: Renderer,
    scene: Scene,
    hud_scene: Scene,
    camera: SharedRef<Camera>,
    hud_camera: SharedRef<Camera>,
    controller: CameraController,
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
            camera.borrow_mut().set_position(&glm::vec3(0.0, 0.5, 3.0));
            scene.add_node(camera);
        }
        let controller =
            CameraController::make_for_camera(&camera).expect("Camera controller is created.");
        {
            let geometry = Geometry::from_with_context(context, BoxGeometry::default())?;
            let material = material::texture::create(
                context,
                Texture::fetch(context, "images/crate.png").await?,
                TextureUnit(0),
                Default::default(),
            )?;
            let crate_mesh = Node::new_with_mesh(Mesh::initialize(context, &geometry, material)?);
            crate_mesh
                .borrow_mut()
                .set_position(&glm::vec3(0.0, 0.5, 0.0));
            scene.add_node(crate_mesh);
        }
        {
            let grid = Node::new_with_mesh(<Rc<Mesh>>::from_with_context(
                context,
                GridHelper {
                    grid_color: color::white(),
                    center_color: color::yellow(),
                    ..Default::default()
                },
            )?);
            grid.borrow_mut().rotate_x(-Angle::RIGHT);
            scene.add_node(grid);
        }
        let (hud_scene, hud_camera) = create_hud(context).await?;
        Ok(Box::new(Example {
            renderer,
            scene,
            hud_scene,
            camera,
            hud_camera,
            controller,
        }))
    }
}

impl Application for Example {
    fn name(&self) -> &str {
        "Heads-Up Display"
    }

    fn update(&mut self, key_state: &KeyState) {
        self.controller.update(key_state);
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera);
        self.renderer.render_clear(
            context,
            &self.hud_scene,
            &self.hud_camera,
            Renderer::CLEAR_DEPTH_ONLY,
            &Default::default(),
        );
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}

async fn create_hud(context: &WebGl2RenderingContext) -> Result<(Scene, SharedRef<Camera>)> {
    let mut scene = Scene::new_empty();
    let camera = Camera::new(Orthographic {
        x_left: 0.0,
        x_right: 800.0,
        y_bottom: 0.0,
        y_top: 600.0,
        z_near: 1.0,
        z_far: -1.0,
    });
    {
        let camera = Node::new_with_camera(Rc::clone(&camera));
        scene.add_node(camera);
    }
    {
        let label1 = Node::new_with_mesh(Mesh::initialize(
            context,
            &Geometry::from_with_context(
                context,
                Rectangle {
                    width: 600.0,
                    height: 80.0,
                    position: glm::vec2(0.0, 600.0),
                    alignment: glm::vec2(0.0, 1.0),
                },
            )?,
            material::texture::create(
                context,
                Texture::fetch(context, "images/crate-sim.png").await?,
                TextureUnit(0),
                Default::default(),
            )?,
        )?);
        scene.add_node(label1);
    }
    {
        let label2 = Node::new_with_mesh(Mesh::initialize(
            context,
            &Geometry::from_with_context(
                context,
                Rectangle {
                    width: 400.0,
                    height: 80.0,
                    position: glm::vec2(800.0, 0.0),
                    alignment: glm::vec2(1.0, 0.0),
                },
            )?,
            material::texture::create(
                context,
                Texture::fetch(context, "images/version-1.png").await?,
                TextureUnit(1),
                Default::default(),
            )?,
        )?);
        scene.add_node(label2);
    }
    Ok((scene, camera))
}
