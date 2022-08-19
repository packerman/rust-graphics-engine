use std::{
    cell::RefCell,
    collections::VecDeque,
    ptr,
    rc::{Rc, Weak},
};

use glm::{vec3, Mat4, Vec3};

use super::matrix::{self, Angle};

pub enum Transform {
    Local,
    Global,
}

pub enum NodeType {
    Group,
}

struct Node {
    me: Weak<Node>,
    transform: RefCell<Mat4>,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
    node_type: NodeType,
}

impl Node {
    pub fn new() -> Rc<Self> {
        Rc::new_cyclic(|me| Node {
            me: me.clone(),
            transform: RefCell::new(matrix::identity()),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
            node_type: NodeType::Group,
        })
    }

    pub fn add_child(&self, child: &Rc<Node>) {
        self.children.borrow_mut().push(Rc::clone(child));
        *child.parent.borrow_mut() = Weak::clone(&self.me);
    }

    pub fn remove_child(&self, child: &Node) {
        if let Some(index) = Self::find_child_index(self, child) {
            self.children.borrow_mut().swap_remove(index);
            drop(child.parent.borrow_mut());
        }
    }

    pub fn world_matrix(&self) -> Mat4 {
        if let Some(parent) = self.parent.borrow().upgrade() {
            parent.world_matrix() * *self.transform.borrow()
        } else {
            *self.transform.borrow()
        }
    }

    pub fn descendants(node: Rc<Node>) -> Vec<Rc<Node>> {
        fn extend_queue(queue: &mut VecDeque<Weak<Node>>, nodes: &[Rc<Node>]) {
            queue.extend(nodes.iter().map(Rc::downgrade));
        }
        fn pop_front(queue: &mut VecDeque<Weak<Node>>) -> Rc<Node> {
            queue.pop_front().unwrap().upgrade().unwrap()
        }
        let mut result = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(Rc::downgrade(&node));
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

    pub fn rotate_z(&self, angle: Angle, transform: Transform) {
        let m = matrix::rotation_z(angle);
        self.appply_matrix(&m, transform);
    }

    pub fn scale(&self, s: f32, transform: Transform) {
        let m = matrix::scale(s);
        self.appply_matrix(&m, transform);
    }

    pub fn get_position(&self) -> Vec3 {
        let transform = self.transform.borrow();
        vec3(transform[(0, 3)], transform[(1, 3)], transform[(2, 3)])
    }

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

    fn find_child_index(parent: &Node, child: &Node) -> Option<usize> {
        parent
            .children
            .borrow()
            .iter()
            .position(|node| ptr::eq(Rc::as_ptr(node), child))
    }
}
