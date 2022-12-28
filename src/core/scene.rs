use std::cell::RefCell;

use web_sys::WebGl2RenderingContext;

use crate::gltf::{program::UpdateUniforms, util::shared_ref::SharedRef};

use super::{camera::Camera, node::Node};

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
