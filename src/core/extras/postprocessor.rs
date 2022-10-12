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
};

pub type Effect = Material;

pub struct Postprocessor<'a> {
    renderer: &'a Renderer,
    scenes: Vec<Rc<Node>>,
    cameras: Vec<Rc<RefCell<Camera>>>,
    render_targets: Vec<Option<RenderTarget>>,
}

impl<'a> Postprocessor<'a> {
    pub fn initialize(
        context: &WebGl2RenderingContext,
        renderer: &'a Renderer,
        scene: Rc<Node>,
        camera: Rc<RefCell<Camera>>,
        render_target: Option<RenderTarget>,
        effects: Vec<Effect>,
    ) -> Result<Self> {
        let geometry = Rc::new(self::create_geometry(context)?);
        let default_camera = Camera::new_ortographic(Default::default());
        let mut scenes = vec![scene];
        let mut cameras = vec![camera];
        let mut render_targets = vec![];
        let resolution = renderer::get_canvas_size(context);
        for effect in effects.into_iter() {
            let target = RenderTarget::initialize(context, resolution)?;
            if let Some(uniform) = effect.uniform("texture0") {
                if let Ok(mut sampler2d) = uniform.sampler2d_mut() {
                    sampler2d.texture = Rc::clone(target.texture());
                }
            }
            scenes.push(self::create_scene(
                context,
                Rc::clone(&geometry),
                effect,
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
