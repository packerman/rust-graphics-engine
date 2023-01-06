use std::{
    cell::{Ref, RefCell},
    collections::VecDeque,
    ptr,
    rc::{Rc, Weak},
};

use glm::{Mat3, Mat4, Vec3};
use web_sys::WebGl2RenderingContext;

use crate::base::{
    math::{angle::Angle, matrix},
    util::{
        cache::Cached,
        shared_ref::{self, SharedRef, WeakRef},
    },
};

use super::{camera::Camera, mesh::Mesh, program::UpdateProgramUniforms};

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
        let node = shared_ref::cyclic(|me| Self {
            me: Weak::clone(me),
            camera: camera.clone(),
            children: vec![],
            local_transform,
            mesh,
            parent: shared_ref::weak(),
            global_transform: Cached::new(),
            normal_transform: Cached::new(),
            name,
        });
        if let Some(camera) = camera {
            camera.borrow_mut().set_node(&node.borrow().me);
        }
        node
    }

    pub fn empty() -> SharedRef<Self> {
        Self::new(glm::identity(), None, None, None)
    }

    pub fn with_name(name: &str) -> SharedRef<Self> {
        Self::new(glm::identity(), None, None, Some(String::from(name)))
    }

    pub fn with_camera_and_name(camera: SharedRef<Camera>, name: &str) -> SharedRef<Self> {
        Self::new(glm::identity(), None, camera.into(), Some(name.into()))
    }

    pub fn with_camera(camera: SharedRef<Camera>) -> SharedRef<Self> {
        Self::new(glm::identity(), None, camera.into(), None)
    }

    pub fn with_mesh(mesh: Rc<Mesh>) -> SharedRef<Self> {
        Self::new(glm::identity(), Some(mesh), None, None)
    }

    pub fn render(
        &self,
        context: &WebGl2RenderingContext,
        view_projection_matrix: &Mat4,
        global_uniform_updater: &dyn UpdateProgramUniforms,
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
        self.reset_transforms();
    }

    pub fn world_position(&self) -> Vec3 {
        matrix::get_position(&self.global_transform())
    }

    pub fn position(&self) -> Vec3 {
        let transform = self.local_transform;
        glm::vec3(transform[(0, 3)], transform[(1, 3)], transform[(2, 3)])
    }

    pub fn set_position(&mut self, position: &Vec3) {
        self.local_transform[(0, 3)] = position[0];
        self.local_transform[(1, 3)] = position[1];
        self.local_transform[(2, 3)] = position[2];
        self.reset_transforms();
    }

    pub fn rotate_x(&mut self, angle: Angle) {
        let m = matrix::rotation_x(angle);
        self.apply_transform(&m);
    }

    pub fn rotate_y(&mut self, angle: Angle) {
        let m = matrix::rotation_y(angle);
        self.apply_transform(&m);
    }

    pub fn look_at(&mut self, target: &Vec3) {
        self.local_transform = matrix::look_at(&self.world_position(), target);
        self.reset_transforms()
    }

    pub fn rotation_matrix(&self) -> Mat3 {
        matrix::get_rotation_matrix(&self.local_transform)
    }

    pub fn direction(&self) -> Vec3 {
        let forward = glm::vec3(0.0, 0.0, -1.0);
        self.rotation_matrix() * forward
    }

    pub fn set_direction(&mut self, direction: &Vec3) {
        let position = self.position();
        let target_position = position + direction;
        self.look_at(&target_position);
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

    pub fn camera(&self) -> Option<&RefCell<Camera>> {
        self.camera.as_deref()
    }

    pub fn mesh(&self) -> Option<&Rc<Mesh>> {
        self.mesh.as_ref()
    }

    pub fn descendants(&self) -> Vec<SharedRef<Node>> {
        fn extend_queue(queue: &mut VecDeque<WeakRef<Node>>, nodes: &[SharedRef<Node>]) {
            queue.extend(nodes.iter().map(Rc::downgrade));
        }
        fn pop_front(queue: &mut VecDeque<WeakRef<Node>>) -> SharedRef<Node> {
            queue.pop_front().unwrap().upgrade().unwrap()
        }
        let mut result = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(self.me.clone());
        while !queue.is_empty() {
            let node = pop_front(&mut queue);
            result.push(Rc::clone(&node));
            extend_queue(&mut queue, &node.borrow().children);
        }
        result
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
