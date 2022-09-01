use glm::Mat4;

use super::matrix::{self, Perspective};

pub struct Camera {
    projection: Perspective,
    view_matrix: Mat4,
}

impl Camera {
    pub fn new(projection: Perspective, view_matrix: Mat4) -> Self {
        Self {
            projection,
            view_matrix,
        }
    }

    pub fn update_view_matrix(&mut self, world_matrix: &Mat4) -> bool {
        if let Some(inverse) = world_matrix.try_inverse() {
            self.view_matrix = inverse;
            true
        } else {
            false
        }
    }

    pub fn set_aspect_ratio(&mut self, width: u32, height: u32) {
        self.projection.set_aspect_ratio(width, height);
    }

    pub fn view_matrix(&self) -> &Mat4 {
        &self.view_matrix
    }

    pub fn projection_matrix(&self) -> Mat4 {
        self.projection.into()
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new(Default::default(), matrix::identity())
    }
}
