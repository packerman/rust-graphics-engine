use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use glm::Mat4;
use web_sys::WebGl2RenderingContext;

use crate::gltf::util::{cache::Cached, shared_ref::SharedRef};

use super::{camera::Camera, geometry::Mesh};

#[derive(Debug, Clone)]
pub struct Node {
    me: Weak<Node>,
    children: RefCell<Vec<Rc<Node>>>,
    local_transform: RefCell<Mat4>,
    camera: Option<SharedRef<Camera>>,
    mesh: Option<Rc<Mesh>>,
    parent: RefCell<Weak<Node>>,
    global_transform: Cached<Mat4>,
}

impl Node {
    pub fn new(
        local_transform: Mat4,
        mesh: Option<Rc<Mesh>>,
        camera: Option<SharedRef<Camera>>,
    ) -> Rc<Self> {
        let node = Rc::new_cyclic(|me| {
            Self(
                Weak::clone(me),
                camera.clone(),
                RefCell::new(vec![]),
                RefCell::new(local_transform),
                mesh,
                RefCell::new(Weak::new()),
                Cached::new(),
            )
        });
        if let Some(camera) = camera {
            camera.borrow().set_node(&node.me);
        }
        node
    }

    pub fn render(&self, context: &WebGl2RenderingContext, view_projection_matrix: &Mat4) {
        if let Some(mesh) = &self.mesh {
            mesh.render(context, self, view_projection_matrix);
        }
    }

    pub fn add_child(&self, node: Rc<Node>) {
        node.set_parent(&self.me);
        self.children.borrow_mut().push(node);
    }

    pub fn global_transform(&self) -> Mat4 {
        self.global_transform.get(|| {
            if let Some(parent) = self.parent.borrow().upgrade() {
                parent.global_transform() * *self.local_transform.borrow()
            } else {
                *self.local_transform.borrow()
            }
        })
    }

    pub fn is_ancestor_of(&self, node: &Rc<Node>) -> bool {
        self.me.upgrade().map_or(false, |me| Rc::ptr_eq(&me, node))
            || self
                .children
                .borrow()
                .iter()
                .any(|child| child.is_ancestor_of(node))
    }

    fn set_parent(&self, parent: &Weak<Node>) {
        *self.parent.borrow_mut() = Weak::clone(parent);
        self.reset_transforms();
    }

    fn reset_transforms(&self) {
        let was_present = self.global_transform.clear();
        if was_present {
            self.children
                .borrow()
                .iter()
                .for_each(|child| child.reset_transforms())
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

    pub fn render(&self, context: &WebGl2RenderingContext, camera: &RefCell<Camera>) {
        let projection_matrix = camera.borrow().projection_matrix();
        let view_matrix = camera.borrow().view_matrix();
        let view_projection_matrix = projection_matrix * view_matrix;
        for node in self.nodes.iter() {
            node.render(context, &view_projection_matrix);
        }
    }

    pub fn contains_node(&self, node: &Rc<Node>) -> bool {
        self.nodes.iter().any(|root| root.is_ancestor_of(node))
    }

    pub fn contains_camera(&self, camera: &RefCell<Camera>) -> bool {
        camera
            .borrow()
            .node()
            .map_or(false, |node| self.contains_node(&node))
    }
}
