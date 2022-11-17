use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::{
    core::{
        application::{self, Application, AsyncCreator},
        camera::Camera,
        convert::FromWithContext,
        geometry::Geometry,
        input::KeyState,
        material::{Material, MaterialSettings},
        mesh::Mesh,
        node::Node,
        renderer::{Renderer, RendererOptions},
        texture::{Texture, TextureData, TextureUnit},
        uniform::data::{Data, Sampler2D},
        web,
    },
    geometry::Rectangle,
};

struct Example {
    renderer: Renderer,
    scene: Rc<Node>,
    camera: Rc<RefCell<Camera>>,
    distort_material: Rc<Material>,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        let renderer = Renderer::initialize(context, RendererOptions::default(), None);
        let scene = Node::new_group();

        let camera = Camera::new_perspective(Default::default());
        {
            let camera = Node::new_camera(Rc::clone(&camera));
            camera.set_position(&glm::vec3(0.0, 0.0, 1.5));
            scene.add_child(&camera);
        }
        let distort_material = Rc::new(Material::from_with_context(
            context,
            MaterialSettings {
                vertex_shader: include_str!("vertex.glsl"),
                fragment_shader: include_str!("fragment.glsl"),
                uniforms: vec![
                    (
                        "noise",
                        Data::from(Sampler2D::new(
                            Texture::initialize(
                                context,
                                TextureData::load_from_source("images/noise.png").await?,
                                Default::default(),
                            )?,
                            TextureUnit::from(0),
                        )),
                    ),
                    (
                        "image",
                        Data::from(Sampler2D::new(
                            Texture::initialize(
                                context,
                                TextureData::load_from_source("images/grid.png").await?,
                                Default::default(),
                            )?,
                            TextureUnit::from(1),
                        )),
                    ),
                    ("time", Data::from(0.0)),
                ],
                render_settings: vec![],
                draw_style: WebGl2RenderingContext::TRIANGLES,
            },
        )?);
        {
            let geometry = Rc::new(Geometry::from_with_context(
                context,
                Rectangle {
                    width: 1.5,
                    height: 1.5,
                    ..Default::default()
                },
            )?);

            let mesh = Node::new_mesh(Mesh::initialize(
                context,
                geometry,
                Rc::clone(&distort_material),
            )?);
            scene.add_child(&mesh);
        }

        Ok(Box::new(Example {
            renderer,
            scene,
            camera,
            distort_material,
        }))
    }
}

impl Application for Example {
    fn update(&mut self, _key_state: &KeyState) {
        if let Some(uniform) = self.distort_material.uniform("time") {
            if let Some(mut time) = uniform.as_mut_float() {
                *time = (web::now().unwrap() / 1000.0) as f32;
            }
        }
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
