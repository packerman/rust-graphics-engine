use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use glm::Mat4;
use web_sys::WebGl2RenderingContext;

use super::{camera::Camera, geometry::Mesh};

#[derive(Debug, Clone)]
pub struct Node {
    me: Weak<Node>,
    children: RefCell<Vec<Rc<Node>>>,
    local_transform: RefCell<Mat4>,
    camera: Option<Rc<RefCell<Camera>>>,
    mesh: Option<Rc<Mesh>>,
    parent: RefCell<Weak<Node>>,
}

impl Node {
    pub fn new(
        local_transform: Mat4,
        mesh: Option<Rc<Mesh>>,
        camera: Option<Rc<RefCell<Camera>>>,
    ) -> Rc<Self> {
        let node = Rc::new_cyclic(|me| Self {
            me: Weak::clone(me),
            camera: camera.clone(),
            children: RefCell::new(vec![]),
            local_transform: RefCell::new(local_transform),
            mesh,
            parent: RefCell::new(Weak::new()),
        });
        if let Some(camera) = camera {
            camera.borrow_mut().set_node(&node.me);
        }
        node
    }

    pub fn render(&self, context: &WebGl2RenderingContext) {
        if let Some(mesh) = &self.mesh {
            mesh.render(context, &self.global_transform());
        }
    }

    pub fn add_child(&self, node: Rc<Node>) {
        *node.parent.borrow_mut() = Weak::clone(&self.me);
        self.children.borrow_mut().push(node);
    }

    pub fn global_transform(&self) -> Mat4 {
        if let Some(parent) = self.parent.borrow().upgrade() {
            parent.global_transform() * *self.local_transform.borrow()
        } else {
            *self.local_transform.borrow()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Scene {
    nodes: Vec<Rc<Node>>,
}

impl Scene {
    pub fn new(nodes: Vec<Rc<Node>>) -> Self {
        Self { nodes }
    }

    pub fn render(&self, context: &WebGl2RenderingContext) {
        for node in self.nodes.iter() {
            node.render(context);
        }
    }
}
