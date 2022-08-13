use std::{
    cell::RefCell,
    ptr,
    rc::{Rc, Weak},
};

use glm::Mat4;

use super::matrix;

struct Node {
    transform: Mat4,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            transform: matrix::identity(),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        }
    }

    pub fn add_child(parent: &Rc<Node>, child: &Rc<Node>) {
        parent.children.borrow_mut().push(Rc::clone(child));
        *child.parent.borrow_mut() = Rc::downgrade(parent);
    }

    pub fn remove_child(&self, child: &Node) {
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
