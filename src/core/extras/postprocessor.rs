use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use web_sys::WebGl2RenderingContext;

use crate::core::{
    attribute::AttributeData,
    camera::Camera,
    convert::FromWithContext,
    geometry::Geometry,
    material::Material,
    mesh::Mesh,
    node::Node,
    render_target::RenderTarget,
    renderer::{self, Renderer},
    texture::TextureUnit,
    uniform::Sampler2D,
};

pub type Effect = Material;

pub struct Postprocessor {
    renderer: Renderer,
    scenes: Vec<Rc<Node>>,
    cameras: Vec<Rc<RefCell<Camera>>>,
    render_targets: Vec<Option<RenderTarget>>,
}

impl Postprocessor {
    pub fn initialize(
        context: &WebGl2RenderingContext,
        renderer: Renderer,
        scene: Rc<Node>,
        camera: Rc<RefCell<Camera>>,
        render_target: Option<RenderTarget>,
        effects: Vec<Box<dyn Fn(&WebGl2RenderingContext, Sampler2D) -> Result<Effect>>>,
        texture_unit: TextureUnit,
    ) -> Result<Self> {
        let geometry = Rc::new(self::create_geometry(context)?);
        let default_camera = Camera::new_ortographic(Default::default());
        let mut scenes = vec![scene];
        let mut cameras = vec![camera];
        let mut render_targets = vec![];
        let resolution = renderer::get_canvas_size(context);
        for effect in effects.into_iter() {
            let target = RenderTarget::initialize(context, resolution)?;
            scenes.push(self::create_scene(
                context,
                Rc::clone(&geometry),
                effect(
                    context,
                    Sampler2D::new(Rc::clone(target.texture()), texture_unit),
                )?,
                Rc::clone(&default_camera),
            )?);
            cameras.push(Rc::clone(&default_camera));
            render_targets.push(Some(target));
        }
        render_targets.push(render_target);

        Ok(Self {
            renderer,
            scenes,
            cameras,
            render_targets,
        })
    }

    pub fn render(&self, context: &WebGl2RenderingContext) {
        for n in 0..self.scenes.len() {
            let scene = &self.scenes[n];
            let camera = &self.cameras[n];
            let target = &self.render_targets[n];
            self.renderer
                .render_to_target(context, scene, camera, target.as_ref())
        }
    }
}

fn create_scene(
    context: &WebGl2RenderingContext,
    geometry: Rc<Geometry>,
    effect: Effect,
    camera: Rc<RefCell<Camera>>,
) -> Result<Rc<Node>> {
    let scene = Node::new_group();
    let mesh = Node::new_mesh(Mesh::initialize(context, geometry, Rc::new(effect))?);
    scene.add_child(&mesh);
    let camera = Node::new_camera(camera);
    scene.add_child(&camera);
    Ok(scene)
}

fn create_geometry(context: &WebGl2RenderingContext) -> Result<Geometry> {
    let p = [[-1.0_f32, -1.0], [1.0, -1.0], [-1.0, 1.0], [1.0, 1.0]];
    let t = [[0.0_f32, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    let position_data = [p[0], p[1], p[3], p[0], p[3], p[2]];
    let uv_data = [t[0], t[1], t[3], t[0], t[3], t[2]];
    Geometry::from_with_context(
        context,
        [
            ("vertexPosition", AttributeData::from(&position_data)),
            ("vertexUV", AttributeData::from(&uv_data)),
        ],
    )
}
