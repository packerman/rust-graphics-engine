pub mod movement_rig;

use std::{
    cell::{Ref, RefCell},
    collections::VecDeque,
    rc::{Rc, Weak},
};

use glm::{Mat3, Mat4, Vec3};

use crate::{
    base::{
        input::KeyState,
        math::{angle::Angle, matrix, resolution::Resolution},
        util::shared_ref::SharedRef,
    },
    core::camera::Camera,
};

use self::movement_rig::MovementRig;

use super::{light::Light, mesh::Mesh};

pub enum Transform {
    Local,
}

impl Default for Transform {
    fn default() -> Self {
        Transform::Local
    }
}

#[derive(Debug, Clone)]
pub enum NodeType {
    Group,
    Mesh(Mesh),
    Camera(SharedRef<Camera>),
    MovementRig(Box<MovementRig>),
    Light(RefCell<Light>),
}

#[derive(Debug, Clone)]
pub struct Node {
    me: Weak<Node>,
    transform: RefCell<Mat4>,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
    node_type: NodeType,
}

impl Node {
    pub fn new_group() -> Rc<Self> {
        Self::new(NodeType::Group)
    }

    pub fn new_mesh(mesh: Mesh) -> Rc<Self> {
        Self::new(NodeType::Mesh(mesh))
    }

    pub fn new_camera(camera: SharedRef<Camera>) -> Rc<Self> {
        Self::new(NodeType::Camera(camera))
    }

    pub fn new_movement_rig(properties: movement_rig::Properties) -> Rc<Self> {
        let look_attachment = Self::new_group();
        let node = Self::new(NodeType::MovementRig(Box::new(MovementRig::new(
            properties,
            Rc::clone(&look_attachment),
        ))));
        node.create_parent_child_relation(&look_attachment);
        node
    }

    pub fn new_light(light: RefCell<Light>) -> Rc<Self> {
        let node = Self::new(NodeType::Light(light));
        node.as_light().unwrap().borrow().update_node(&node);
        node
    }

    pub fn new(node_type: NodeType) -> Rc<Self> {
        Rc::new_cyclic(|me| Node {
            me: me.clone(),
            transform: RefCell::new(matrix::identity()),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
            node_type,
        })
    }

    pub fn as_mesh(&self) -> Option<&Mesh> {
        match &self.node_type {
            NodeType::Mesh(mesh) => Some(mesh),
            _ => None,
        }
    }

    pub fn as_camera(&self) -> Option<&Rc<RefCell<Camera>>> {
        match &self.node_type {
            NodeType::Camera(camera) => Some(camera),
            _ => None,
        }
    }

    pub fn as_light(&self) -> Option<&RefCell<Light>> {
        match &self.node_type {
            NodeType::Light(light) => Some(light),
            _ => None,
        }
    }

    pub fn add_child(&self, child: &Rc<Node>) {
        match &self.node_type {
            NodeType::MovementRig(movement_rig) => movement_rig.add_child(child),
            _ => self.create_parent_child_relation(child),
        }
    }

    pub fn world_matrix(&self) -> Mat4 {
        if let Some(parent) = self.parent.borrow().upgrade() {
            parent.world_matrix() * *self.transform.borrow()
        } else {
            *self.transform.borrow()
        }
    }

    pub fn descendants(&self) -> Vec<Rc<Node>> {
        fn extend_queue(queue: &mut VecDeque<Weak<Node>>, nodes: &[Rc<Node>]) {
            queue.extend(nodes.iter().map(Rc::downgrade));
        }
        fn pop_front(queue: &mut VecDeque<Weak<Node>>) -> Rc<Node> {
            queue.pop_front().unwrap().upgrade().unwrap()
        }
        let mut result = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(self.me.clone());
        while !queue.is_empty() {
            let node = pop_front(&mut queue);
            result.push(Rc::clone(&node));
            extend_queue(&mut queue, &node.children.borrow());
        }
        result
    }

    pub fn apply_matrix(&self, matrix: &Mat4, transform: Transform) {
        match transform {
            Transform::Local => self.transform.replace_with(|&mut old| old * matrix),
        };
    }

    pub fn update_key_state(&self, key_state: &KeyState) {
        if let NodeType::MovementRig(movement_rig) = &self.node_type {
            movement_rig.update(key_state, self)
        }
    }

    pub fn update(&self) {
        match &self.node_type {
            // NodeType::Camera(camera) => {
            //     camera.borrow_mut().update_world_matrix(self.world_matrix());
            // }
            NodeType::Light(light) => {
                light.borrow_mut().update_from_node(self);
            }
            _ => {}
        }
    }

    pub fn update_resolution(&self, resolution: &Resolution) {
        if let NodeType::Camera(camera) = &self.node_type {
            camera.borrow_mut().set_aspect_ratio(resolution.ratio());
        }
    }

    pub fn translate(&self, x: f32, y: f32, z: f32, transform: Transform) {
        let m = matrix::translation(x, y, z);
        self.apply_matrix(&m, transform);
    }

    pub fn rotate_x(&self, angle: Angle, transform: Transform) {
        let m = matrix::rotation_x(angle);
        self.apply_matrix(&m, transform);
    }

    pub fn rotate_y(&self, angle: Angle, transform: Transform) {
        let m = matrix::rotation_y(angle);
        self.apply_matrix(&m, transform);
    }

    pub fn position(&self) -> Vec3 {
        let transform = self.transform.borrow();
        glm::vec3(transform[(0, 3)], transform[(1, 3)], transform[(2, 3)])
    }

    pub fn world_position(&self) -> Vec3 {
        matrix::get_position(&self.world_matrix())
    }

    pub fn set_position(&self, position: &Vec3) {
        let mut transform = self.transform.borrow_mut();
        transform[(0, 3)] = position[0];
        transform[(1, 3)] = position[1];
        transform[(2, 3)] = position[2];
    }

    pub fn look_at(&self, target: &Vec3) {
        *self.transform.borrow_mut() = matrix::look_at(&self.world_position(), target);
    }

    pub fn rotation_matrix(&self) -> Mat3 {
        matrix::get_rotation_matrix(&self.transform.borrow())
    }

    pub fn direction(&self) -> Vec3 {
        let forward = glm::vec3(0.0, 0.0, -1.0);
        self.rotation_matrix() * forward
    }

    pub fn set_direction(&self, direction: &Vec3) {
        let position = self.position();
        let target_position = position + direction;
        self.look_at(&target_position);
    }

    pub fn transform(&self) -> Ref<Mat4> {
        self.transform.borrow()
    }

    pub fn set_transform(&self, m: &Mat4) {
        *self.transform.borrow_mut() = *m;
    }

    fn create_parent_child_relation(&self, child: &Rc<Node>) {
        self.children.borrow_mut().push(Rc::clone(child));
        *child.parent.borrow_mut() = Weak::clone(&self.me);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_position_works() {
        let node = Node::new_group();
        node.apply_matrix(&matrix::translation(3.0, 5.0, 1.0), Default::default());
        assert_eq!(node.world_position(), glm::vec3(3.0, 5.0, 1.0));
    }

    #[test]
    fn look_at_works() {
        let node = Node::new_group();
        node.apply_matrix(&matrix::translation(0.0, 1.0, 0.0), Default::default());
        assert_eq!(
            node.world_matrix(),
            glm::mat4(
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0
            )
        );
        node.look_at(&glm::vec3(0.0, 1.0, 5.0));
        assert_eq!(
            node.world_matrix(),
            glm::mat4(
                -1.0, 0.0, -0.0, 0.0, 0.0, 1.0, -0.0, 1.0, 0.0, -0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0
            )
        );
        assert_eq!(node.world_position(), glm::vec3(0.0, 1.0, 0.0));
        node.look_at(&glm::vec3(0.0, 1.0, 5.0));
        assert_eq!(
            node.world_matrix(),
            glm::mat4(
                -1.0, 0.0, -0.0, 0.0, 0.0, 1.0, -0.0, 1.0, 0.0, -0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0
            )
        );
    }
}