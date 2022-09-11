use std::{
    cell::RefCell,
    collections::VecDeque,
    ptr,
    rc::{Rc, Weak},
};

use glm::{vec3, Mat4, Vec3};

use super::{
    camera::Camera,
    extras::movement_rig::{self, MovementRig},
    input::KeyState,
    matrix::{self, Angle},
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

pub enum NodeKind<'a> {
    Group,
    Mesh(Box<Mesh<'a>>),
    Camera(Rc<RefCell<Camera>>),
    MovementRig(Box<MovementRig<'a>>),
}

pub struct Node<'a> {
    me: Weak<Node<'a>>,
    transform: RefCell<Mat4>,
    parent: RefCell<Weak<Node<'a>>>,
    children: RefCell<Vec<Rc<Node<'a>>>>,
    kind: NodeKind<'a>,
}

impl<'a> Node<'a> {
    pub fn new_group() -> Rc<Self> {
        Self::new(NodeKind::Group)
    }

    pub fn new_mesh(mesh: Box<Mesh<'a>>) -> Rc<Self> {
        Self::new(NodeKind::Mesh(mesh))
    }

    pub fn new_camera(camera: Rc<RefCell<Camera>>) -> Rc<Self> {
        Self::new(NodeKind::Camera(camera))
    }

    pub fn new_movement_rig(properties: movement_rig::Properties) -> Rc<Self> {
        let look_attachment = Self::new_group();
        let node = Self::new(NodeKind::MovementRig(Box::new(MovementRig::new(
            properties,
            Rc::clone(&look_attachment),
        ))));
        node.create_parent_child_relation(&look_attachment);
        node
    }

    pub fn new(node_type: NodeKind<'a>) -> Rc<Self> {
        Rc::new_cyclic(|me| Node {
            me: me.clone(),
            transform: RefCell::new(matrix::identity()),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
            kind: node_type,
        })
    }

    pub fn mesh(&self) -> Option<&Mesh<'a>> {
        match &self.kind {
            NodeKind::Mesh(mesh) => Some(mesh),
            _ => None,
        }
    }

    pub fn camera(&self) -> Option<&RefCell<Camera>> {
        match &self.kind {
            NodeKind::Camera(camera) => Some(camera),
            _ => None,
        }
    }

    pub fn add_child(&self, child: &Rc<Node<'a>>) {
        match &self.kind {
            NodeKind::MovementRig(movement_rig) => movement_rig.add_child(child),
            _ => self.create_parent_child_relation(child),
        }
    }

    #[allow(dead_code)]
    pub fn remove_child(&self, child: &Node<'a>) {
        match &self.kind {
            NodeKind::MovementRig(movement_rig) => movement_rig.remove_child(child),
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

    pub fn descendants(&self) -> Vec<Rc<Node<'a>>> {
        fn extend_queue<'a>(queue: &mut VecDeque<Weak<Node<'a>>>, nodes: &[Rc<Node<'a>>]) {
            queue.extend(nodes.iter().map(Rc::downgrade));
        }
        fn pop_front<'a>(queue: &mut VecDeque<Weak<Node<'a>>>) -> Rc<Node<'a>> {
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

    pub fn appply_matrix(&self, matrix: &Mat4, transform: Transform) {
        match transform {
            Transform::Local => self.transform.replace_with(|&mut old| old * matrix),
            Transform::Global => self.transform.replace_with(|&mut old| matrix * old),
        };
    }

    pub fn update(&self, key_state: &KeyState) {
        if let NodeKind::MovementRig(movement_rig) = &self.kind {
            movement_rig.update(key_state, self)
        }
    }

    pub fn translate(&self, x: f32, y: f32, z: f32, transform: Transform) {
        let m = matrix::translation(x, y, z);
        self.appply_matrix(&m, transform);
    }

    pub fn rotate_x(&self, angle: Angle, transform: Transform) {
        let m = matrix::rotation_x(angle);
        self.appply_matrix(&m, transform);
    }

    pub fn rotate_y(&self, angle: Angle, transform: Transform) {
        let m = matrix::rotation_y(angle);
        self.appply_matrix(&m, transform);
    }

    #[allow(dead_code)]
    pub fn rotate_z(&self, angle: Angle, transform: Transform) {
        let m = matrix::rotation_z(angle);
        self.appply_matrix(&m, transform);
    }

    #[allow(dead_code)]
    pub fn scale(&self, s: f32, transform: Transform) {
        let m = matrix::scale(s);
        self.appply_matrix(&m, transform);
    }

    #[allow(dead_code)]
    pub fn get_position(&self) -> Vec3 {
        let transform = self.transform.borrow();
        vec3(transform[(0, 3)], transform[(1, 3)], transform[(2, 3)])
    }

    #[allow(dead_code)]
    pub fn get_world_position(&self) -> Vec3 {
        let transform = self.world_matrix();
        vec3(transform[(0, 3)], transform[(1, 3)], transform[(2, 3)])
    }

    pub fn set_position(&self, position: &Vec3) {
        let mut transform = self.transform.borrow_mut();
        transform[(0, 3)] = position[0];
        transform[(1, 3)] = position[1];
        transform[(2, 3)] = position[2];
    }

    fn create_parent_child_relation(&self, child: &Rc<Node<'a>>) {
        self.children.borrow_mut().push(Rc::clone(child));
        *child.parent.borrow_mut() = Weak::clone(&self.me);
    }

    fn remove_parent_child_relation(&self, child: &Node<'a>) {
        if let Some(index) = Self::find_child_index(self, child) {
            self.children.borrow_mut().swap_remove(index);
            drop(child.parent.borrow_mut());
        }
    }

    fn find_child_index(parent: &Node<'a>, child: &Node<'a>) -> Option<usize> {
        parent
            .children
            .borrow()
            .iter()
            .position(|node| ptr::eq(Rc::as_ptr(node), child))
    }
}
