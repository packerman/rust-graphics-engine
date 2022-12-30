use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::{
    api::geometry::Geometry,
    base::{
        application::{self, Application, AsyncCreator},
        convert::FromWithContext,
        input::KeyState,
        web,
    },
    core::{
        camera::Camera,
        material::{Material, ProgramCreator},
        mesh::Mesh,
        node::Node,
        program::UpdateProgramUniforms,
        texture::{Texture, TextureUnit},
    },
    geometry::Rectangle,
    legacy::{
        renderer::{Renderer, RendererOptions},
        texture::Sampler2D,
    },
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
            DistortMaterial {
                noise: Sampler2D::new(Texture::fetch(context, "images/noise.png")?, TextureUnit(0)),
                image: Sampler2D::new(Texture::fetch(context, "images/grid.png")?, TextureUnit(1)),
                time: 0.0, // TODO
            }, // MaterialSettings {
               //     vertex_shader: include_str!("vertex.glsl"),
               //     fragment_shader: include_str!("fragment.glsl"),
               //     uniforms: vec![
               //         (
               //             "noise",
               //             Data::from(Sampler2D::new(
               //                 Texture::initialize(
               //                     context,
               //                     TextureData::load_from_source("images/noise.png").await?,
               //                     Default::default(),
               //                 )?,
               //                 TextureUnit(0),
               //             )),
               //         ),
               //         (
               //             "image",
               //             Data::from(Sampler2D::new(
               //                 Texture::initialize(
               //                     context,
               //                     TextureData::load_from_source("images/grid.png").await?,
               //                     Default::default(),
               //                 )?,
               //                 TextureUnit(1),
               //             )),
               //         ),
               //         ("time", Data::from(0.0)),
               //     ],
               //     render_settings: vec![],
               //     draw_style: WebGl2RenderingContext::TRIANGLES,
               // },
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

#[derive(Debug, Clone)]
struct DistortMaterial {
    noise: Sampler2D,
    image: Sampler2D,
    time: f32,
}

impl ProgramCreator for DistortMaterial {
    fn vertex_shader(&self) -> &str {
        todo!()
    }

    fn fragment_shader(&self) -> &str {
        todo!()
    }
}

impl UpdateProgramUniforms for DistortMaterial {
    fn update_program_uniforms(
        &self,
        context: &WebGl2RenderingContext,
        program: &crate::core::program::Program,
    ) {
        todo!()
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
