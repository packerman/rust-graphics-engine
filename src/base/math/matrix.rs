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

pub fn get_position(m: &Mat4) -> Vec3 {
    glm::vec3(m[(0, 3)], m[(1, 3)], m[(2, 3)])
}

pub fn get_rotation_matrix(m: &Mat4) -> Mat3 {
    glm::mat4_to_mat3(m)
}
