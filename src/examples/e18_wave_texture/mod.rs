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
        util::shared_ref::{self, SharedRef},
        web,
    },
    core::{
        camera::{Camera, Perspective},
        material::{GenericMaterial, Material},
        mesh::Mesh,
        node::Node,
        program::{Program, UpdateProgramUniforms, UpdateUniform},
        scene::Scene,
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
    scene: Scene,
    camera: Rc<RefCell<Camera>>,
    wave_material: SharedRef<WaveMaterial>,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        let renderer = Renderer::initialize(context, RendererOptions::default(), None);
        let mut scene = Scene::new_empty();

        let camera = shared_ref::strong(Camera::from(Perspective::default()));
        {
            let camera = Node::new_with_camera(Rc::clone(&camera));
            camera.borrow_mut().set_position(&glm::vec3(0.0, 0.0, 1.5));
            scene.add_root_node(camera);
        }
        let wave_material = shared_ref::strong(WaveMaterial {
            texture_sampler: Sampler2D::new(
                Rc::new(Texture::fetch(context, "images/grid.png").await?),
                TextureUnit(0),
            ),
            time: 0.0,
        });
        {
            let geometry = Geometry::from_with_context(
                context,
                Rectangle {
                    width: 1.5,
                    height: 1.5,
                    ..Default::default()
                },
            )?;

            let mesh = Node::new_with_mesh(Rc::new(Mesh::initialize(
                context,
                &geometry,
                Rc::new(Material::from_with_context(
                    context,
                    Rc::clone(&wave_material),
                )?),
            )?));
            scene.add_root_node(mesh);
        }

        Ok(Box::new(Example {
            renderer,
            scene,
            camera,
            wave_material,
        }))
    }
}

impl Application for Example {
    fn update(&mut self, _key_state: &KeyState) {
        self.wave_material.borrow_mut().time = (web::now().unwrap() / 1000.0) as f32;
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}

#[derive(Debug, Clone)]
struct WaveMaterial {
    texture_sampler: Sampler2D,
    time: f32,
}

impl GenericMaterial for WaveMaterial {
    fn vertex_shader(&self) -> &str {
        include_str!("vertex.glsl")
    }

    fn fragment_shader(&self) -> &str {
        include_str!("fragment.glsl")
    }
}

impl UpdateProgramUniforms for WaveMaterial {
    fn update_program_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {
        self.texture_sampler
            .update_uniform(context, "textureSampler", program);
        self.time.update_uniform(context, "time", program);
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
