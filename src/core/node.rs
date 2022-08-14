use std::{
    cell::RefCell,
    collections::VecDeque,
    ptr,
    rc::{Rc, Weak},
};

use glm::Mat4;

use super::matrix;

struct Node {
    me: Weak<Node>,
    transform: Mat4,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    pub fn new() -> Rc<Self> {
        Rc::new_cyclic(|me| Node {
            me: me.clone(),
            transform: matrix::identity(),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        })
    }

    pub fn add_child(&self, child: &Rc<Node>) {
        self.children.borrow_mut().push(Rc::clone(child));
        *child.parent.borrow_mut() = self.me.clone();
    }

    pub fn remove_child(&self, child: &Node) {
        if let Some(index) = Self::find_child_index(self, child) {
            self.children.borrow_mut().swap_remove(index);
            drop(child.parent.borrow_mut());
        }
    }

    pub fn world_matrix(&self) -> Mat4 {
        if let Some(parent) = self.parent.borrow().upgrade() {
            parent.world_matrix() * self.transform
        } else {
            self.transform
        }
    }

    pub fn descendants(node: Rc<Node>) -> Vec<Rc<Node>> {
        fn extend_queue(queue: &mut VecDeque<Weak<Node>>, nodes: &Vec<Rc<Node>>) {
            queue.extend(nodes.iter().map(|child| Rc::downgrade(&child)));
        }
        let mut result = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(Rc::downgrade(&node));
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap().upgrade().unwrap();
            result.push(Rc::clone(&node));
            extend_queue(&mut queue, &node.children.borrow());
        }
        result
    }

    fn find_child_index(parent: &Node, child: &Node) -> Option<usize> {
        parent
            .children
            .borrow()
            .iter()
            .position(|node| ptr::eq(Rc::as_ptr(node), child))
    }
}
