pub mod movement_rig;

use std::{
    cell::RefCell,
    collections::VecDeque,
    ptr,
    rc::{Rc, Weak},
};

use glm::{Mat3, Mat4, Vec3};

use crate::light::Light;

use self::movement_rig::MovementRig;

use super::{
    camera::Camera,
    input::KeyState,
    math::{angle::Angle, matrix},
    mesh::Mesh,
};

#[allow(dead_code)]
pub enum Transform {
    Local,
    Global,
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
    Camera(Rc<RefCell<Camera>>),
    MovementRig(Box<MovementRig>),
    Light(Light),
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

    pub fn new_camera(camera: Rc<RefCell<Camera>>) -> Rc<Self> {
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

    pub fn new_light(light: Light) -> Rc<Self> {
        Self::new(NodeType::Light(light))
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

    pub fn mesh(&self) -> Option<&Mesh> {
        match &self.node_type {
            NodeType::Mesh(mesh) => Some(mesh),
            _ => None,
        }
    }

    pub fn camera(&self) -> Option<&RefCell<Camera>> {
        match &self.node_type {
            NodeType::Camera(camera) => Some(camera),
            _ => None,
        }
    }

    pub fn light(&self) -> Option<&Light> {
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

    #[allow(dead_code)]
    pub fn remove_child(&self, child: &Node) {
        match &self.node_type {
            NodeType::MovementRig(movement_rig) => movement_rig.remove_child(child),
            _ => self.remove_parent_child_relation(child),
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
            Transform::Global => self.transform.replace_with(|&mut old| matrix * old),
        };
    }

    pub fn update(&self, key_state: &KeyState) {
        if let NodeType::MovementRig(movement_rig) = &self.node_type {
            movement_rig.update(key_state, self)
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

    #[allow(dead_code)]
    pub fn rotate_z(&self, angle: Angle, transform: Transform) {
        let m = matrix::rotation_z(angle);
        self.apply_matrix(&m, transform);
    }

    #[allow(dead_code)]
    pub fn scale(&self, s: f32, transform: Transform) {
        let m = matrix::scale(s);
        self.apply_matrix(&m, transform);
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn rotation_matrix(&self) -> Mat3 {
        matrix::get_rotation_matrix(&self.transform.borrow())
    }

    #[allow(dead_code)]
    pub fn direction(&self) -> Vec3 {
        let forward = glm::vec3(0.0, 0.0, -1.0);
        self.rotation_matrix() * forward
    }

    #[allow(dead_code)]
    pub fn set_direction(&self, direction: &Vec3) {
        let position = self.position();
        let target_position = position + direction;
        self.look_at(&target_position);
    }

    fn create_parent_child_relation(&self, child: &Rc<Node>) {
        self.children.borrow_mut().push(Rc::clone(child));
        *child.parent.borrow_mut() = Weak::clone(&self.me);
    }

    fn remove_parent_child_relation(&self, child: &Node) {
        if let Some(index) = Self::find_child_index(self, child) {
            self.children.borrow_mut().swap_remove(index);
            drop(child.parent.borrow_mut());
        }
    }

    fn find_child_index(parent: &Node, child: &Node) -> Option<usize> {
        parent
            .children
            .borrow()
            .iter()
            .position(|node| ptr::eq(Rc::as_ptr(node), child))
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
