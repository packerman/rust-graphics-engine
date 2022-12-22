use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::{
    base::{
        application::{self, Application, AsyncCreator},
        color,
        convert::FromWithContext,
        input::KeyState,
        math::{angle::Angle, resolution::Resolution},
    },
    core::{
        camera::Camera,
        geometry::Geometry,
        material::Material,
        mesh::Mesh,
        node::Node,
        render_target::RenderTarget,
        renderer::{Renderer, RendererOptions},
        texture::{Texture, TextureData},
    },
    geometry::{parametric::Sphere, BoxGeometry, Rectangle},
    gltf::core::texture_data::TextureUnit,
    material::{
        self,
        basic::{BasicMaterial, SurfaceMaterial},
        texture::TextureMaterial,
    },
};

struct Example {
    renderer: Renderer,
    scene: Rc<Node>,
    rig: Rc<Node>,
    camera: Rc<RefCell<Camera>>,
    sky_camera: Rc<RefCell<Camera>>,
    sphere: Rc<Node>,
    render_target: RenderTarget,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        let renderer = Renderer::initialize(context, RendererOptions::default(), None);
        let scene = Node::new_group();

        let camera = Camera::new_perspective(Default::default());
        let rig = Node::new_movement_rig(Default::default());
        {
            let camera = Node::new_camera(Rc::clone(&camera));
            rig.add_child(&camera);
            scene.add_child(&rig);
            rig.set_position(&glm::vec3(0.0, 1.0, 4.0));
        }
        {
            let sky = Node::new_mesh(Mesh::initialize(
                context,
                Rc::new(Geometry::from_with_context(
                    context,
                    Sphere {
                        radius: 50.0,
                        ..Default::default()
                    },
                )?),
                material::texture::create(
                    context,
                    Texture::initialize(
                        context,
                        TextureData::load_from_source("images/sky-earth.jpg").await?,
                        Default::default(),
                    )?,
                    TextureUnit(0),
                    Default::default(),
                )?,
            )?);
            scene.add_child(&sky);
        }
        {
            let grass = Node::new_mesh(Mesh::initialize(
                context,
                Rc::new(Geometry::from_with_context(
                    context,
                    Rectangle {
                        width: 100.0,
                        height: 100.0,
                        ..Default::default()
                    },
                )?),
                material::texture::create(
                    context,
                    Texture::initialize(
                        context,
                        TextureData::load_from_source("images/grass.jpg").await?,
                        Default::default(),
                    )?,
                    TextureUnit(1),
                    TextureMaterial {
                        repeat_uv: glm::vec2(50.0, 50.0),
                        ..Default::default()
                    },
                )?,
            )?);
            grass.rotate_x(-Angle::RIGHT, Default::default());
            scene.add_child(&grass);
        }
        let sphere = Node::new_mesh(Mesh::initialize(
            context,
            Rc::new(Geometry::from_with_context(context, Sphere::default())?),
            material::texture::create(
                context,
                Texture::initialize(
                    context,
                    TextureData::load_from_source("images/grid.png").await?,
                    Default::default(),
                )?,
                TextureUnit(2),
                Default::default(),
            )?,
        )?);
        {
            sphere.set_position(&glm::vec3(-1.2, 1.0, 0.0));
            scene.add_child(&sphere);
        }
        {
            let box_mesh = Node::new_mesh(Mesh::initialize(
                context,
                Rc::new(Geometry::from_with_context(
                    context,
                    BoxGeometry {
                        width: 2.0,
                        height: 2.0,
                        depth: 0.2,
                    },
                )?),
                Rc::new(Material::from_with_context(
                    context,
                    SurfaceMaterial {
                        basic: BasicMaterial {
                            base_color: color::black(),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                )?),
            )?);
            box_mesh.set_position(&glm::vec3(1.2, 1.0, 0.0));
            scene.add_child(&box_mesh);
        }
        let render_target = RenderTarget::initialize(context, Resolution::new(512, 512))?;
        let screen = Node::new_mesh(Mesh::initialize(
            context,
            Rc::new(Geometry::from_with_context(
                context,
                Rectangle {
                    width: 1.8,
                    height: 1.8,
                    ..Default::default()
                },
            )?),
            material::texture::create(
                context,
                render_target.texture(),
                TextureUnit(3),
                Default::default(),
            )?,
        )?);
        {
            screen.set_position(&glm::vec3(1.2, 1.0, 0.11));
            scene.add_child(&screen);
        }
        let sky_camera = Camera::new_perspective(Default::default());
        {
            let sky_camera = Node::new_camera(Rc::clone(&sky_camera));
            sky_camera.set_position(&glm::vec3(0.0, 10.0, 0.1));
            sky_camera.look_at(&glm::vec3(0.0, 0.0, 0.0));
            scene.add_child(&sky_camera);
        }

        Ok(Box::new(Example {
            renderer,
            rig,
            scene,
            camera,
            sky_camera,
            sphere,
            render_target,
        }))
    }
}

impl Application for Example {
    fn update(&mut self, key_state: &KeyState) {
        self.sphere
            .rotate_y(Angle::STRAIGHT / 235.0, Default::default());
        self.rig.update_key_state(key_state);
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render_to_target(
            context,
            &self.scene,
            &self.sky_camera,
            Some(&self.render_target),
        );
        self.renderer.render(context, &self.scene, &self.camera);
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
