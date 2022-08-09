use glm::Mat4;

pub fn identity() -> Mat4 {
    glm::identity()
}

pub fn translation(x: f32, y: f32, z: f32) -> Mat4 {
    glm::translation(&glm::vec3(x, y, z))
}

pub fn rotation_x(angle: f32) -> Mat4 {
    glm::rotation(angle, &glm::vec3(1.0, 0.0, 0.0))
}

pub fn rotation_y(angle: f32) -> Mat4 {
    glm::rotation(angle, &glm::vec3(0.0, 1.0, 0.0))
}

pub fn rotation_z(angle: f32) -> Mat4 {
    glm::rotation(angle, &glm::vec3(0.0, 0.0, 1.0))
}

pub fn scale(s: f32) -> Mat4 {
    glm::scaling(&glm::vec3(s, s, s))
}

pub struct Perspective {
    pub aspect_ratio: f32,
    pub angle_of_view_degrees: f32,
    pub near: f32,
    pub far: f32,
}

impl Perspective {
    pub fn set_aspect_ratio<T>(&mut self, width: T, height: T)
    where
        T: Into<f32>,
    {
        self.aspect_ratio = width.into() / height.into()
    }
}

impl Default for Perspective {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            angle_of_view_degrees: 60.0,
            near: 0.1,
            far: 1000.0,
        }
    }
}

impl From<Perspective> for Mat4 {
    fn from(perspective: Perspective) -> Self {
        glm::perspective(
            perspective.aspect_ratio,
            perspective.angle_of_view_degrees.to_radians(),
            perspective.near,
            perspective.far,
        )
    }
}
