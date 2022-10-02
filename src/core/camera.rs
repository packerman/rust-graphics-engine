use glm::Mat4;

use super::matrix::{self, Ortographic, Perspective};

#[derive(Debug, Clone, Copy)]
enum Projection {
    Perspective(Perspective),
    Ortographic(Ortographic),
}

impl From<Projection> for Mat4 {
    fn from(projection: Projection) -> Self {
        match projection {
            Projection::Perspective(perspective) => perspective.into(),
            Projection::Ortographic(ortographic) => ortographic.into(),
        }
    }
}

pub struct Camera {
    projection: Projection,
    view_matrix: Mat4,
}

impl Camera {
    pub fn new_perspective(projection: Perspective) -> Self {
        Self {
            projection: Projection::Perspective(projection),
            view_matrix: matrix::identity(),
        }
    }

    pub fn new_ortographic(projection: Ortographic) -> Self {
        Self {
            projection: Projection::Ortographic(projection),
            view_matrix: matrix::identity(),
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
        if let Projection::Perspective(perspective) = &mut self.projection {
            perspective.set_aspect_ratio(width, height);
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
        Camera::new_perspective(Default::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn aspect_ratio(camera: &Camera) -> Option<f32> {
        if let Projection::Perspective(perspective) = camera.projection {
            Some(perspective.aspect_ratio)
        } else {
            None
        }
    }

    #[test]
    fn set_aspect_ratio_works() {
        let mut camera = Camera::default();
        assert_eq!(aspect_ratio(&camera).unwrap(), 1.0);
        camera.set_aspect_ratio(800, 600);
        assert_eq!(aspect_ratio(&camera).unwrap(), 1.3333334);
    }
}
