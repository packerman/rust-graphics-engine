use std::{cell::RefCell, rc::Rc};

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

#[derive(Debug, Clone)]
pub struct Camera {
    projection: Projection,
    view_matrix: Mat4,
}

impl Camera {
    fn new(projection: Projection) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            projection,
            view_matrix: matrix::identity(),
        }))
    }

    pub fn new_perspective(projection: Perspective) -> Rc<RefCell<Self>> {
        Self::new(Projection::Perspective(projection))
    }

    pub fn new_ortographic(projection: Ortographic) -> Rc<RefCell<Self>> {
        Self::new(Projection::Ortographic(projection))
    }

    pub fn update_view_matrix(&mut self, world_matrix: &Mat4) -> bool {
        if let Some(inverse) = world_matrix.try_inverse() {
            self.view_matrix = inverse;
            true
        } else {
            false
        }
    }

    pub fn set_aspect_ratio(&mut self, resolution: (i32, i32)) {
        if let Projection::Perspective(perspective) = &mut self.projection {
            perspective.aspect_ratio = resolution.0 as f32 / resolution.1 as f32;
        }
    }

    pub fn view_matrix(&self) -> &Mat4 {
        &self.view_matrix
    }

    pub fn projection_matrix(&self) -> Mat4 {
        self.projection.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn aspect_ratio(camera: &RefCell<Camera>) -> Option<f32> {
        if let Projection::Perspective(perspective) = camera.borrow().projection {
            Some(perspective.aspect_ratio)
        } else {
            None
        }
    }

    #[test]
    fn set_aspect_ratio_works() {
        let camera = Camera::new_perspective(Default::default());
        assert_eq!(aspect_ratio(&camera).unwrap(), 1.0);
        camera.borrow_mut().set_aspect_ratio((800, 600));
        assert_eq!(aspect_ratio(&camera).unwrap(), 1.3333334);
    }
}
