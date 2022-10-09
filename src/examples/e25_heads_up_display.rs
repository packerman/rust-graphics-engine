use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::core::{
    application::{self, Application, AsyncCreator},
    camera::Camera,
    color::Color,
    convert::FromWithContext,
    extras::GridHelper,
    geometry::{BoxGeometry, Geometry, Rectangle},
    input::KeyState,
    material,
    matrix::{Angle, Ortographic},
    mesh::Mesh,
    node::Node,
    renderer::{Renderer, RendererOptions},
    texture::{Texture, TextureData, TextureUnit},
};

struct Example {
    renderer: Renderer,
    scene: Rc<Node>,
    hud_scene: Rc<Node>,
    camera: Rc<RefCell<Camera>>,
    hud_camera: Rc<RefCell<Camera>>,
    rig: Rc<Node>,
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

        let camera = Camera::new_perspective(Default::default());
        let rig = Node::new_movement_rig(Default::default());
        {
            rig.set_position(&glm::vec3(0.0, 0.5, 3.0));
            let camera = Node::new_camera(Rc::clone(&camera));
            rig.add_child(&camera);
            scene.add_child(&rig);
        }
        {
            let geometry = <Box<Geometry>>::from_with_context(context, BoxGeometry::default())?;
            let material = material::texture::create(
                context,
                Texture::initialize(
                    context,
                    TextureData::load_from_source("images/crate.png").await?,
                    Default::default(),
                )?,
                TextureUnit::from(0),
                Default::default(),
            )?;
            let crate_mesh = Node::new_mesh(Mesh::initialize(context, geometry, material)?);
            crate_mesh.set_position(&glm::vec3(0.0, 0.5, 0.0));
            scene.add_child(&crate_mesh);
        }
        {
            let grid = Node::new_mesh(Mesh::from_with_context(
                context,
                GridHelper {
                    grid_color: Color::white(),
                    center_color: Color::yellow(),
                    ..Default::default()
                },
            )?);
            grid.rotate_x(-Angle::RIGHT, Default::default());
            scene.add_child(&grid);
        }
        let (hud_scene, hud_camera) = create_hud(context).await?;
        Ok(Example {
            renderer,
            scene,
            hud_scene,
            camera,
            hud_camera,
            rig,
        })
    }
}

impl Application for Example {
    fn update(&mut self, key_state: &KeyState) {
        self.rig.update(key_state);
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera);
        self.renderer.render_clear(
            context,
            &self.hud_scene,
            &self.hud_camera,
            Renderer::CLEAR_DEPTH_ONLY,
        );
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}

async fn create_hud(context: &WebGl2RenderingContext) -> Result<(Rc<Node>, Rc<RefCell<Camera>>)> {
    let scene = Node::new_group();
    let camera = Camera::new_ortographic(Ortographic {
        left: 0.0,
        right: 800.0,
        bottom: 0.0,
        top: 600.0,
        near: 1.0,
        far: -1.0,
    });
    {
        let camera = Node::new_camera(Rc::clone(&camera));
        scene.add_child(&camera);
    }
    {
        let label1 = Node::new_mesh(Mesh::initialize(
            context,
            <Box<Geometry>>::from_with_context(
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
                Texture::initialize(
                    context,
                    TextureData::load_from_source("images/crate-sim.png").await?,
                    Default::default(),
                )?,
                TextureUnit::from(0),
                Default::default(),
            )?,
        )?);
        scene.add_child(&label1);
    }
    {
        let label2 = Node::new_mesh(Mesh::initialize(
            context,
            <Box<Geometry>>::from_with_context(
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
                Texture::initialize(
                    context,
                    TextureData::load_from_source("images/version-1.png").await?,
                    Default::default(),
                )?,
                TextureUnit::from(1),
                Default::default(),
            )?,
        )?);
        scene.add_child(&label2);
    }
    Ok((scene, camera))
}
