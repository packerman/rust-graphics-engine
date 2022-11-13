use std::rc::Rc;

use anyhow::Result;
use glm::Vec3;
use web_sys::WebGl2RenderingContext;

use crate::{
    core::{
        camera::Camera,
        material::Material,
        math::{matrix::Ortographic, resolution::Resolution},
        node::Node,
        render_target::RenderTarget,
    },
    material,
};

#[derive(Debug, Clone, Copy)]
pub struct CameraBounds {
    pub min: Vec3,
    pub max: Vec3,
}

impl Default for CameraBounds {
    fn default() -> Self {
        Self {
            min: glm::vec3(-5.0, -5.0, 0.0),
            max: glm::vec3(5.0, 5.0, 20.0),
        }
    }
}

impl From<CameraBounds> for Ortographic {
    fn from(camera_bounds: CameraBounds) -> Self {
        Ortographic {
            left: camera_bounds.min.x,
            right: camera_bounds.max.x,
            bottom: camera_bounds.min.y,
            top: camera_bounds.max.y,
            near: camera_bounds.min.z,
            far: camera_bounds.max.z,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ShadowOptions {
    strength: f32,
    camera_bounds: CameraBounds,
    bias: f32,
}

impl Default for ShadowOptions {
    fn default() -> Self {
        Self {
            strength: 0.5,
            camera_bounds: Default::default(),
            bias: 0.01,
        }
    }
}

pub struct Shadow {
    light_source: Rc<Node>,
    resolution: Resolution,
    options: ShadowOptions,
    camera: Rc<Node>,
    render_target: RenderTarget,
    material: Rc<Material>,
}

impl Shadow {
    pub fn initialize(
        context: &WebGl2RenderingContext,
        light_source: Rc<Node>,
        resolution: Resolution,
        options: ShadowOptions,
    ) -> Result<Self> {
        assert!(light_source
            .light()
            .map_or(false, |light| light.borrow().is_directional()));
        let camera = Node::new_camera(Camera::new_ortographic(options.camera_bounds.into()));
        light_source.add_child(&camera);

        let render_target = RenderTarget::initialize(context, resolution)?;

        let material = material::depth::create(context)?;

        Ok(Self {
            light_source,
            resolution,
            options,
            camera,
            render_target,
            material,
        })
    }

    pub fn update_internal(&self) {
        self.camera.update();
        if let Some(camera) = self.camera.camera() {
            let camera = camera.borrow();
            if let Some(mut view_matrix) = self.material.mat4_mut("viewMatrix") {
                *view_matrix = *camera.view_matrix();
            }
            if let Some(mut projection_matrix) = self.material.mat4_mut("projectionMatrix") {
                *projection_matrix = camera.projection_matrix();
            }
        }
    }
}
