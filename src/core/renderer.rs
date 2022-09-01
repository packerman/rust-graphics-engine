use web_sys::WebGl2RenderingContext;

use super::{
    camera::Camera,
    color::Color,
    gl,
    node::{Node, NodeType},
};

pub struct RendererOptions {
    pub clear_color: Color,
}

impl Default for RendererOptions {
    fn default() -> Self {
        Self {
            clear_color: Color::gray(),
        }
    }
}

pub struct Renderer;

impl Renderer {
    pub fn new_initialized(context: &WebGl2RenderingContext, options: RendererOptions) -> Self {
        context.enable(WebGl2RenderingContext::DEPTH_TEST);
        gl::set_clear_color(context, &options.clear_color);
        Self
    }

    pub fn render(&self, context: &WebGl2RenderingContext, scene: &Node, camera: &Node) {
        context.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );
        let nodes = scene.descendants();

        for node in nodes.iter() {
            if let Some(camera) = node.camera() {
                camera.borrow_mut().update_view_matrix(&node.world_matrix());
            }
        }

        let camera = &camera.camera().unwrap().borrow();
        for node in nodes.iter() {
            if let Some(mesh) = node.mesh() {
                mesh.render(context, camera, node.world_matrix())
            }
        }
    }
}
