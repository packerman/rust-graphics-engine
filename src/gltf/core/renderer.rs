use std::cell::RefCell;

use web_sys::WebGl2RenderingContext;

use crate::core::{
    color::{self, Color},
    gl,
};

use super::{camera::Camera, scene::Scene};

#[derive(Debug, Clone)]
pub struct Renderer {
    clear_color: Color,
}

impl Renderer {
    pub fn initialize(context: &WebGl2RenderingContext) -> Self {
        context.enable(WebGl2RenderingContext::DEPTH_TEST);
        Renderer::default()
    }

    pub fn render(
        &self,
        context: &WebGl2RenderingContext,
        scene: &Scene,
        camera: &RefCell<Camera>,
    ) {
        context.viewport(
            0,
            0,
            context.drawing_buffer_width(),
            context.drawing_buffer_height(),
        );
        gl::set_clear_color(context, &self.clear_color);
        context.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );
        scene.render(context, camera)
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self {
            clear_color: color::gray(),
        }
    }
}
