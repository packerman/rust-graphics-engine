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
        math::{angle::Angle, resolution::Resolution},
        util::shared_ref::{self, SharedRef},
    },
    classic::{
        render_target::RenderTarget,
        renderer::{Renderer, RendererOptions},
    },
    core::{
        camera::{Camera, Perspective},
        material::Material,
        mesh::Mesh,
        node::Node,
        scene::Scene,
        texture::{Texture, TextureUnit},
    },
    extras::camera_controller::CameraController,
    geometry::{box_geom::BoxGeometry, parametric::Sphere, rectangle::Rectangle},
    material::{
        self,
        basic::{BasicMaterial, SurfaceMaterial},
        texture::Properties,
    },
};

struct Example {
    renderer: Renderer,
    scene: Scene,
    rig: CameraController,
    camera: SharedRef<Camera>,
    sky_camera: SharedRef<Camera>,
    sphere: SharedRef<Node>,
    render_target: RenderTarget,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        let renderer = Renderer::initialize(context, RendererOptions::default(), None);
        let mut scene = Scene::new_empty();

        let camera = Camera::new(Perspective::default());
        {
            let camera = Node::new_with_camera(Rc::clone(&camera));
            camera.borrow_mut().set_position(&glm::vec3(0.0, 1.0, 4.0));
            scene.add_node(camera);
        }
        let rig =
            CameraController::make_for_camera(&camera).expect("Camera controller is created.");
        {
            let sky = Node::new_with_mesh(Mesh::initialize(
                context,
                &Geometry::from_with_context(
                    context,
                    Sphere {
                        radius: 50.0,
                        ..Default::default()
                    },
                )?,
                material::texture::create(
                    context,
                    Texture::fetch(context, "images/sky-earth.jpg").await?,
                    TextureUnit(0),
                    Default::default(),
                )?,
            )?);
            scene.add_node(sky);
        }
        {
            let grass = Node::new_with_mesh(Mesh::initialize(
                context,
                &Geometry::from_with_context(
                    context,
                    Rectangle {
                        width: 100.0,
                        height: 100.0,
                        ..Default::default()
                    },
                )?,
                material::texture::create(
                    context,
                    Texture::fetch(context, "images/grass.jpg").await?,
                    TextureUnit(1),
                    Properties {
                        repeat_uv: glm::vec2(50.0, 50.0),
                        ..Default::default()
                    },
                )?,
            )?);
            grass.borrow_mut().rotate_x(-Angle::RIGHT);
            scene.add_node(grass);
        }
        let sphere = Node::new_with_mesh(Mesh::initialize(
            context,
            &Geometry::from_with_context(context, Sphere::default())?,
            material::texture::create(
                context,
                Texture::fetch(context, "images/grid.png").await?,
                TextureUnit(2),
                Default::default(),
            )?,
        )?);
        {
            sphere.borrow_mut().set_position(&glm::vec3(-1.2, 1.0, 0.0));
            scene.add_node(Rc::clone(&sphere));
        }
        {
            let box_mesh = Node::new_with_mesh(Mesh::initialize(
                context,
                &Geometry::from_with_context(
                    context,
                    BoxGeometry {
                        width: 2.0,
                        height: 2.0,
                        depth: 0.2,
                    },
                )?,
                <Rc<Material>>::from_with_context(
                    context,
                    shared_ref::new(SurfaceMaterial {
                        basic: BasicMaterial {
                            base_color: color::black(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
                )?,
            )?);
            box_mesh
                .borrow_mut()
                .set_position(&glm::vec3(1.2, 1.0, 0.0));
            scene.add_node(box_mesh);
        }
        let render_target = RenderTarget::initialize(context, Resolution::new(512, 512))?;
        let screen = Node::new_with_mesh(Mesh::initialize(
            context,
            &Geometry::from_with_context(
                context,
                Rectangle {
                    width: 1.8,
                    height: 1.8,
                    ..Default::default()
                },
            )?,
            material::texture::create(
                context,
                render_target.texture(),
                TextureUnit(3),
                Default::default(),
            )?,
        )?);
        {
            screen.borrow_mut().set_position(&glm::vec3(1.2, 1.0, 0.11));
            scene.add_node(screen);
        }
        let sky_camera = Camera::new(Perspective::default());
        {
            let sky_camera = Node::new_with_camera(Rc::clone(&sky_camera));
            sky_camera
                .borrow_mut()
                .set_position(&glm::vec3(0.0, 10.0, 0.1));
            sky_camera.borrow_mut().look_at(&glm::vec3(0.0, 0.0, 0.0));
            scene.add_node(sky_camera);
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
    fn name(&self) -> &str {
        "Render to texture"
    }

    fn update(&mut self, key_state: &KeyState) {
        self.sphere.borrow_mut().rotate_y(Angle::STRAIGHT / 235.0);
        self.rig.update(key_state);
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render_to_target(
            context,
            &self.scene,
            &self.sky_camera,
            Some(&self.render_target),
            &[],
        );
        self.renderer.render(context, &self.scene, &self.camera);
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
