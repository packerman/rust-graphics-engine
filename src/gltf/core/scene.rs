use std::{
    cell::RefCell,
    ptr,
    rc::{Rc, Weak},
};

use glm::Mat4;
use web_sys::WebGl2RenderingContext;

use crate::gltf::util::{
    cache::Cached,
    shared_ref::{SharedRef, WeakRef},
};

use super::{camera::Camera, geometry::Mesh};

#[derive(Debug, Clone)]
pub struct Node {
    me: WeakRef<Node>,
    children: Vec<SharedRef<Node>>,
    local_transform: Mat4,
    camera: Option<SharedRef<Camera>>,
    mesh: Option<Rc<Mesh>>,
    parent: WeakRef<Node>,
    global_transform: Cached<Mat4>,
}

impl Node {
    pub fn new(
        local_transform: Mat4,
        mesh: Option<Rc<Mesh>>,
        camera: Option<SharedRef<Camera>>,
    ) -> SharedRef<Self> {
        let node = SharedRef::new_cyclic(|me| Self {
            me: Weak::clone(me),
            camera: camera.clone(),
            children: vec![],
            local_transform,
            mesh,
            parent: Weak::new(),
            global_transform: Cached::new(),
        });
        if let Some(camera) = camera {
            camera.borrow_mut().set_node(&node.borrow().me);
        }
        node
    }

    pub fn render(&self, context: &WebGl2RenderingContext, view_projection_matrix: &Mat4) {
        if let Some(mesh) = &self.mesh {
            mesh.render(context, self, view_projection_matrix);
        }
    }

    pub fn add_child(&mut self, node: SharedRef<Node>) {
        node.borrow_mut().set_parent(&self.me);
        self.children.push(node);
    }

    pub fn global_transform(&self) -> Mat4 {
        self.global_transform.get(|| {
            if let Some(parent) = self.parent.upgrade() {
                parent.borrow().global_transform() * self.local_transform
            } else {
                self.local_transform
            }
        })
    }

    pub fn is_ancestor_of(&self, node: &RefCell<Node>) -> bool {
        self.me
            .upgrade()
            .map_or(false, |me| ptr::eq(me.as_ptr(), node.as_ptr()))
            || self
                .children
                .iter()
                .any(|child| child.borrow().is_ancestor_of(node))
    }

    pub fn has_some_camera(&self) -> bool {
        self.camera.is_some()
            || self
                .children
                .iter()
                .any(|child| child.borrow().has_some_camera())
    }

    fn set_parent(&mut self, parent: &WeakRef<Node>) {
        self.parent = Weak::clone(parent);
        self.reset_transforms();
    }

    fn reset_transforms(&self) {
        let was_present = self.global_transform.clear();
        if was_present {
            self.children
                .iter()
                .for_each(|child| child.borrow().reset_transforms())
        }
    }
}

#[derive(Debug, Clone)]
pub struct Scene {
    nodes: Vec<SharedRef<Node>>,
}

impl Scene {
    pub fn new(nodes: Vec<SharedRef<Node>>) -> Self {
        Self { nodes }
    }

    pub fn render(&self, context: &WebGl2RenderingContext, camera: &RefCell<Camera>) {
        let projection_matrix = camera.borrow().projection_matrix();
        let view_matrix = camera.borrow().view_matrix();
        let view_projection_matrix = projection_matrix * view_matrix;
        for node in self.nodes.iter() {
            node.borrow().render(context, &view_projection_matrix);
        }
    }

    pub fn contains_node(&self, node: &RefCell<Node>) -> bool {
        self.nodes
            .iter()
            .any(|root| root.borrow().is_ancestor_of(node))
    }

    pub fn add_root_node(&mut self, node: SharedRef<Node>) {
        self.nodes.push(node)
    }

    pub fn contains_camera(&self, camera: &RefCell<Camera>) -> bool {
        camera
            .borrow()
            .node()
            .map_or(false, |node| self.contains_node(&node))
    }

    pub fn has_some_camera(&self) -> bool {
        self.nodes
            .iter()
            .any(|node| node.borrow().has_some_camera())
    }
}
