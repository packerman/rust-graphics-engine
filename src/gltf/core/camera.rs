use std::rc::Weak;

use glm::Mat4;

use crate::gltf::util::shared_ref::{SharedRef, WeakRef};

use super::scene::Node;

#[derive(Debug, Clone)]
pub struct Camera {
    camera_type: CameraType,
    #[allow(dead_code)]
    name: Option<String>,
    node: WeakRef<Node>,
}

impl Camera {
    pub fn perspective(
        aspect_ratio: f32,
        y_fov: f32,
        z_near: f32,
        z_far: Option<f32>,
        name: Option<String>,
    ) -> Self {
        Self {
            name,
            camera_type: CameraType::Perspective(Perspective {
                aspect_ratio,
                y_fov,
                z_far,
                z_near,
            }),
            node: Weak::new(),
        }
    }

    pub fn orthographic(
        x_mag: f32,
        y_mag: f32,
        z_near: f32,
        z_far: f32,
        name: Option<String>,
    ) -> Self {
        Self {
            name,
            camera_type: CameraType::Orthographic(Orthographic {
                x_mag,
                y_mag,
                z_far,
                z_near,
            }),
            node: Weak::new(),
        }
    }

    pub fn default_with_name(name: &str) -> Self {
        let mut camera = Self::default();
        camera.name = Some(name.into());
        camera
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
                -orthographic.x_mag,
                orthographic.x_mag,
                -orthographic.y_mag,
                orthographic.y_mag,
                orthographic.z_near,
                orthographic.z_far,
            ),
        }
    }

    pub fn view_matrix(&self) -> Mat4 {
        if let Some(node) = self.node.upgrade() {
            if let Some(inverse) = node.borrow().global_transform().try_inverse() {
                inverse
            } else {
                glm::identity()
            }
        } else {
            glm::identity()
        }
    }

    pub fn node(&self) -> Option<SharedRef<Node>> {
        SharedRef::upgrade(&self.node)
    }

    pub fn set_node(&mut self, node: &WeakRef<Node>) {
        self.node = Weak::clone(node);
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        if let CameraType::Perspective(perspective) = &mut self.camera_type {
            perspective.aspect_ratio = aspect_ratio;
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::orthographic(1.0, 1.0, 1.0, -1.0, None)
    }
}

#[derive(Debug, Clone)]
pub enum CameraType {
    Orthographic(Orthographic),
    Perspective(Perspective),
}

#[derive(Debug, Clone)]
pub struct Orthographic {
    x_mag: f32,
    y_mag: f32,
    z_far: f32,
    z_near: f32,
}

#[derive(Debug, Clone)]
pub struct Perspective {
    aspect_ratio: f32,
    y_fov: f32,
    z_far: Option<f32>,
    z_near: f32,
}
