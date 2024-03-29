use std::rc::Weak;

use glm::{Mat4, Vec3};

use crate::base::{
    math::matrix,
    util::{
        level::Level,
        shared_ref::{self, SharedRef, WeakRef},
    },
};

use super::{
    node::Node,
    program::{UpdateProgramUniforms, UpdateUniform},
};

#[derive(Debug, Clone)]
pub struct Camera {
    camera_type: CameraType,
    #[allow(dead_code)]
    name: Option<String>,
    node: WeakRef<Node>,
}

impl Camera {
    pub fn new<T>(camera_type: T) -> SharedRef<Self>
    where
        T: Into<CameraType>,
    {
        Self::new_with_name(camera_type.into(), None)
    }

    pub fn new_with_name(camera_type: CameraType, name: Option<String>) -> SharedRef<Self> {
        shared_ref::new(Self {
            camera_type,
            name,
            node: shared_ref::weak(),
        })
    }

    pub fn perspective(
        aspect_ratio: f32,
        y_fov: f32,
        z_near: f32,
        z_far: Option<f32>,
        name: Option<String>,
    ) -> SharedRef<Self> {
        Self::new_with_name(
            CameraType::Perspective(Perspective {
                aspect_ratio,
                y_fov,
                z_far,
                z_near,
            }),
            name,
        )
    }

    pub fn orthographic(
        x_mag: f32,
        y_mag: f32,
        z_near: f32,
        z_far: f32,
        name: Option<String>,
    ) -> SharedRef<Self> {
        Self::new_with_name(
            CameraType::Orthographic(Orthographic {
                x_left: -x_mag,
                x_right: x_mag,
                y_bottom: -y_mag,
                y_top: y_mag,
                z_far,
                z_near,
            }),
            name,
        )
    }

    pub fn projection_matrix(&self) -> Mat4 {
        match &self.camera_type {
            CameraType::Perspective(perspective) => {
                if let Some(z_far) = perspective.z_far {
                    glm::perspective(
                        perspective.aspect_ratio,
                        perspective.y_fov,
                        perspective.z_near,
                        z_far,
                    )
                } else {
                    glm::infinite_perspective_rh_no(
                        perspective.aspect_ratio,
                        perspective.y_fov,
                        perspective.z_near,
                    )
                }
            }
            CameraType::Orthographic(orthographic) => glm::ortho(
                orthographic.x_left,
                orthographic.x_right,
                orthographic.y_bottom,
                orthographic.y_top,
                orthographic.z_near,
                orthographic.z_far,
            ),
        }
    }

    pub fn model_matrix(&self) -> Mat4 {
        if let Some(node) = self.node() {
            node.borrow().global_transform()
        } else {
            matrix::identity()
        }
    }

    pub fn view_matrix(&self) -> Mat4 {
        if let Some(inverse) = self.model_matrix().try_inverse() {
            inverse
        } else {
            matrix::identity()
        }
    }

    pub fn view_projection_matrix(&self) -> Mat4 {
        let projection_matrix = self.projection_matrix();
        let view_matrix = self.view_matrix();
        projection_matrix * view_matrix
    }

    pub fn world_position(&self) -> Vec3 {
        matrix::get_position(&self.model_matrix())
    }

    pub fn node(&self) -> Option<SharedRef<Node>> {
        self.node.upgrade()
    }

    pub fn set_node(&mut self, node: &WeakRef<Node>) {
        self.node = Weak::clone(node);
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        if let CameraType::Perspective(perspective) = &mut self.camera_type {
            perspective.aspect_ratio = aspect_ratio;
        }
    }

    pub fn matrix(&self) -> CameraMatrix {
        let view = self.view_matrix();
        let projection = self.projection_matrix();
        let view_projection = projection * view;
        CameraMatrix {
            projection,
            view,
            view_projection,
        }
    }
}

#[derive(Debug, Clone)]
pub enum CameraType {
    Orthographic(Orthographic),
    Perspective(Perspective),
}

impl From<Orthographic> for CameraType {
    fn from(value: Orthographic) -> Self {
        Self::Orthographic(value)
    }
}

impl From<Perspective> for CameraType {
    fn from(value: Perspective) -> Self {
        Self::Perspective(value)
    }
}

#[derive(Debug, Clone)]
pub struct Orthographic {
    pub x_left: f32,
    pub x_right: f32,
    pub y_bottom: f32,
    pub y_top: f32,
    pub z_far: f32,
    pub z_near: f32,
}

impl Default for Orthographic {
    fn default() -> Self {
        Self {
            x_left: -1.0,
            x_right: 1.0,
            y_bottom: -1.0,
            y_top: 1.0,
            z_far: 1.0,
            z_near: -1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Perspective {
    pub aspect_ratio: f32,
    pub y_fov: f32,
    pub z_far: Option<f32>,
    pub z_near: f32,
}

impl Default for Perspective {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            y_fov: 60_f32.to_radians(),
            z_near: 0.1,
            z_far: Some(1000.0),
        }
    }
}

#[derive(Debug)]
pub struct CameraMatrix {
    projection: Mat4,
    view: Mat4,
    view_projection: Mat4,
}

impl UpdateProgramUniforms for CameraMatrix {
    fn update_program_uniforms(
        &self,
        context: &web_sys::WebGl2RenderingContext,
        program: &super::program::Program,
    ) {
        self.projection.update_uniform_with_level(
            context,
            "u_ProjectionMatrix",
            program,
            Level::Ignore,
        );
        self.view
            .update_uniform_with_level(context, "u_ViewMatrix", program, Level::Ignore);
        self.view_projection.update_uniform_with_level(
            context,
            "u_ViewProjectionMatrix",
            program,
            Level::Ignore,
        );
    }
}
