use glm::{Mat3, Mat4, Vec3};

use super::angle::Angle;

pub fn identity() -> Mat4 {
    glm::identity()
}

pub fn translation(x: f32, y: f32, z: f32) -> Mat4 {
    glm::translation(&glm::vec3(x, y, z))
}

pub fn rotation_x(angle: Angle) -> Mat4 {
    glm::rotation(angle.to_radians(), &glm::vec3(1.0, 0.0, 0.0))
}

pub fn rotation_y(angle: Angle) -> Mat4 {
    glm::rotation(angle.to_radians(), &glm::vec3(0.0, 1.0, 0.0))
}

pub fn rotation_z(angle: Angle) -> Mat4 {
    glm::rotation(angle.to_radians(), &glm::vec3(0.0, 0.0, 1.0))
}

#[allow(dead_code)]
pub fn scale(s: f32) -> Mat4 {
    glm::scaling(&glm::vec3(s, s, s))
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

pub fn look_at(position: &Vec3, target: &Vec3) -> Mat4 {
    let world_up = glm::vec3(0.0, 1.0, 0.0);
    let forward = target - position;
    let right = forward.cross(&world_up);
    assert!(right.norm_squared() > 0.0);
    let up = right.cross(&forward);
    let forward = forward.normalize();
    let right = right.normalize();
    let up = up.normalize();
    glm::mat4(
        right[0],
        up[0],
        -forward[0],
        position[0],
        right[1],
        up[1],
        -forward[1],
        position[1],
        right[2],
        up[2],
        -forward[2],
        position[2],
        0.0,
        0.0,
        0.0,
        1.0,
    )
}

pub fn get_rotation_matrix(m: &Mat4) -> Mat3 {
    glm::mat4_to_mat3(m)
}
