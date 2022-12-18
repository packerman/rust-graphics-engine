use std::{
    cell::{Ref, RefCell},
    ptr,
    rc::{Rc, Weak},
};

use glm::Mat4;
use web_sys::WebGl2RenderingContext;

use crate::gltf::{
    program::UpdateUniforms,
    util::{
        cache::Cached,
        shared_ref::{SharedRef, WeakRef},
    },
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
    normal_transform: Cached<Mat4>,
    #[allow(dead_code)]
    name: Option<String>,
}

impl Node {
    pub fn new(
        local_transform: Mat4,
        mesh: Option<Rc<Mesh>>,
        camera: Option<SharedRef<Camera>>,
        name: Option<String>,
    ) -> SharedRef<Self> {
        let node = SharedRef::new_cyclic(|me| Self {
            me: Weak::clone(me),
            camera: camera.clone(),
            children: vec![],
            local_transform,
            mesh,
            parent: Weak::new(),
            global_transform: Cached::new(),
            normal_transform: Cached::new(),
            name,
        });
        if let Some(camera) = camera {
            camera.borrow_mut().set_node(&node.borrow().me);
        }
        node
    }

    pub fn with_name(name: &str) -> SharedRef<Self> {
        Self::new(glm::identity(), None, None, Some(String::from(name)))
    }

    pub fn with_camera_and_name(camera: SharedRef<Camera>, name: &str) -> SharedRef<Self> {
        Self::new(glm::identity(), None, camera.into(), Some(name.into()))
    }

    pub fn render(
        &self,
        context: &WebGl2RenderingContext,
        view_projection_matrix: &Mat4,
        global_uniform_updater: &dyn UpdateUniforms,
    ) {
        if let Some(mesh) = &self.mesh {
            mesh.render(
                context,
                self,
                view_projection_matrix,
                global_uniform_updater,
            );
        }
        for child in self.children.iter() {
            child
                .borrow()
                .render(context, view_projection_matrix, global_uniform_updater)
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

    pub fn normal_transform(&self) -> Mat4 {
        self.normal_transform
            .get(|| self.global_transform().try_inverse().unwrap().transpose())
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

    pub fn depth(&self) -> usize {
        Self::max_by_key(&self.children, |child| child.depth() + 1)
    }

    pub fn apply_transform(&mut self, transform: &Mat4) {
        self.local_transform *= transform;
        self.reset_transforms()
    }

    pub fn transfer_camera(&mut self, destination: &RefCell<Self>) {
        if let Some(camera) = self.camera.take() {
            destination.borrow_mut().attach_camera(camera)
        }
    }

    pub fn attach_camera(&mut self, camera: SharedRef<Camera>) {
        let camera = self.camera.insert(camera);
        camera.borrow_mut().set_node(&self.me);
    }

    pub fn max_by_key<K>(nodes: &[SharedRef<Node>], key: K) -> usize
    where
        K: Fn(Ref<Node>) -> usize,
    {
        nodes
            .iter()
            .map(|node| key(node.borrow()))
            .max()
            .unwrap_or_default()
    }

    fn set_parent(&mut self, parent: &WeakRef<Node>) {
        self.parent = Weak::clone(parent);
        self.reset_transforms();
    }

    fn reset_transforms(&self) {
        self.normal_transform.clear();
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

    pub fn render(
        &self,
        context: &WebGl2RenderingContext,
        camera: &RefCell<Camera>,
        global_uniform_updater: &dyn UpdateUniforms,
    ) {
        let projection_matrix = camera.borrow().projection_matrix();
        let view_matrix = camera.borrow().view_matrix();
        let view_projection_matrix = projection_matrix * view_matrix;
        for node in self.nodes.iter() {
            node.borrow()
                .render(context, &view_projection_matrix, global_uniform_updater);
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

    pub fn depth(&self) -> usize {
        Node::max_by_key(&self.nodes, |node| node.depth())
    }
}
