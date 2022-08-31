use glm::Mat4;

use super::matrix::{self, Perspective};

pub struct Camera {
    projection: Perspective,
    view_matrix: Mat4,
}

impl Camera {
    pub fn update_view_matrix(&mut self, world_matrix: &Mat4) -> bool {
        if let Some(inverse) = world_matrix.try_inverse() {
            self.view_matrix = inverse;
            true
        } else {
            false
        }
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
        Self {
            projection: Default::default(),
            view_matrix: matrix::identity(),
        }
    }
}
