use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use web_sys::WebGl2RenderingContext;

use crate::{
    api::geometry::Geometry,
    base::{convert::FromWithContext, math::resolution::Resolution, util::shared_ref::SharedRef},
    core::{
        accessor::Accessor,
        camera::{Camera, Orthographic},
        material::Material,
        mesh::{self, Mesh},
        node::Node,
        scene::Scene,
        texture::{Texture, TextureUnit},
    },
    legacy::{
        light::Light,
        render_target::RenderTarget,
        renderer::{self, Renderer},
        texture::Sampler2D,
    },
};

pub type Effect = Material;

pub struct Postprocessor {
    renderer: Rc<Renderer>,
    scenes: Vec<Scene>,
    cameras: Vec<Rc<RefCell<Camera>>>,
    render_targets: Vec<Option<RenderTarget>>,
    resolution: Resolution,
    geometry: Rc<Geometry>,
    texture_unit: TextureUnit,
    default_camera: Rc<RefCell<Camera>>,
}

impl Postprocessor {
    pub fn initialize(
        context: &WebGl2RenderingContext,
        renderer: Rc<Renderer>,
        scene: Scene,
        camera: SharedRef<Camera>,
        render_target: Option<RenderTarget>,
        texture_unit: TextureUnit,
    ) -> Result<Self> {
        Ok(Self {
            renderer,
            scenes: vec![scene],
            cameras: vec![camera],
            render_targets: vec![render_target],
            resolution: renderer::get_canvas_resolution(context),
            geometry: Rc::new(self::create_geometry(context)?),
            texture_unit,
            default_camera: Camera::new(Orthographic::default()),
        })
    }

    pub fn add_effect<E>(&mut self, context: &WebGl2RenderingContext, effect: E) -> Result<()>
    where
        E: Fn(Sampler2D) -> Result<Rc<Effect>>,
    {
        let target = RenderTarget::initialize(context, self.resolution)?;
        self.scenes.push(self::create_scene(
            context,
            Rc::clone(&self.geometry),
            effect(Sampler2D::new(target.texture(), self.texture_unit))?,
            Rc::clone(&self.default_camera),
        )?);
        self.cameras.push(Rc::clone(&self.default_camera));
        let final_render_target = self.render_targets.pop().flatten();
        self.render_targets.push(Some(target));
        self.render_targets.push(final_render_target);
        Ok(())
    }

    pub fn render(&self, context: &WebGl2RenderingContext, lights: &[Light]) {
        for n in 0..self.scenes.len() {
            let scene = &self.scenes[n];
            let camera = &self.cameras[n];
            let target = &self.render_targets[n];
            self.renderer
                .render_to_target(context, scene, camera, target.as_ref(), lights)
        }
    }

    pub fn get_texture(&self, index: usize) -> Option<Rc<Texture>> {
        self.render_targets[index]
            .as_ref()
            .map(|render_target| render_target.texture())
    }
}

fn create_scene(
    context: &WebGl2RenderingContext,
    geometry: Rc<Geometry>,
    effect: Rc<Effect>,
    camera: SharedRef<Camera>,
) -> Result<Scene> {
    let mut scene = Scene::new_empty();
    let mesh = Node::new_with_mesh(Mesh::initialize(context, geometry.as_ref(), effect)?);
    scene.add_root_node(mesh);
    let camera = Node::new_with_camera(camera);
    scene.add_root_node(camera);
    Ok(scene)
}

fn create_geometry(context: &WebGl2RenderingContext) -> Result<Geometry> {
    let p = [[-1.0_f32, -1.0], [1.0, -1.0], [-1.0, 1.0], [1.0, 1.0]];
    let t = [[0.0_f32, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    let position_data = [p[0], p[1], p[3], p[0], p[3], p[2]];
    let uv_data = [t[0], t[1], t[3], t[0], t[3], t[2]];
    let geometry = Geometry::from([
        (
            mesh::POSITION_ATTRIBUTE,
            Rc::new(Accessor::from_with_context(context, &position_data)?),
        ),
        (
            mesh::TEXCOORD_0_ATTRIBUTE,
            Rc::new(Accessor::from_with_context(context, &uv_data)?),
        ),
    ]);
    Ok(geometry)
}
