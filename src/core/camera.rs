use std::{cell::RefCell, rc::Rc};

use glm::{Mat4, Vec3};

use crate::base::math::{angle::Angle, matrix};

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
    world_matrix: Mat4,
    view_matrix: Mat4,
}

impl Camera {
    fn new(projection: Projection) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            projection,
            world_matrix: matrix::identity(),
            view_matrix: matrix::identity(),
        }))
    }

    pub fn new_perspective(projection: Perspective) -> Rc<RefCell<Self>> {
        Self::new(Projection::Perspective(projection))
    }

    pub fn new_ortographic(projection: Ortographic) -> Rc<RefCell<Self>> {
        Self::new(Projection::Ortographic(projection))
    }

    pub fn update_world_matrix(&mut self, world_matrix: Mat4) -> bool {
        if let Some(inverse) = world_matrix.try_inverse() {
            self.world_matrix = world_matrix;
            self.view_matrix = inverse;
            true
        } else {
            false
        }
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        if let Projection::Perspective(perspective) = &mut self.projection {
            perspective.aspect_ratio = aspect_ratio;
        }
    }

    pub fn view_matrix(&self) -> &Mat4 {
        &self.view_matrix
    }

    pub fn projection_matrix(&self) -> Mat4 {
        self.projection.into()
    }

    pub fn world_position(&self) -> Vec3 {
        matrix::get_position(&self.world_matrix)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Perspective {
    pub aspect_ratio: f32,
    pub angle_of_view: Angle,
    pub near: f32,
    pub far: f32,
}

impl Default for Perspective {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            angle_of_view: Angle::from_degrees(60.0),
            near: 0.1,
            far: 1000.0,
        }
    }
}

impl From<Perspective> for Mat4 {
    fn from(perspective: Perspective) -> Self {
        glm::perspective(
            perspective.aspect_ratio,
            perspective.angle_of_view.to_radians(),
            perspective.near,
            perspective.far,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ortographic {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,
}

impl Default for Ortographic {
    fn default() -> Self {
        Self {
            left: -1.0,
            right: 1.0,
            bottom: -1.0,
            top: 1.0,
            near: -1.0,
            far: 1.0,
        }
    }
}

impl From<Ortographic> for Mat4 {
    fn from(orto: Ortographic) -> Self {
        glm::ortho(
            orto.left,
            orto.right,
            orto.bottom,
            orto.top,
            orto.near,
            orto.far,
        )
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
        camera.borrow_mut().set_aspect_ratio(1.3333334);
        assert_eq!(aspect_ratio(&camera).unwrap(), 1.3333334);
    }
}
