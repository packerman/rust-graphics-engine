use std::cell::RefCell;

use web_sys::WebGl2RenderingContext;

use crate::{
    base::{
        color::{self, Color},
        gl, web,
    },
    gltf::program::UpdateUniforms,
};

use super::{camera::Camera, scene::Scene};

#[derive(Debug, Clone)]
pub struct Properties {
    clear_color: Color,
}

impl Default for Properties {
    fn default() -> Self {
        Self {
            clear_color: color::gray(),
        }
    }
}

#[derive(Debug)]
pub struct Renderer {
    properties: Properties,
    global_uniform_updater: Box<dyn UpdateUniforms>,
}

impl Renderer {
    pub fn initialize(
        context: &WebGl2RenderingContext,
        properties: Properties,
        global_uniform_updater: Box<dyn UpdateUniforms>,
    ) -> Self {
        context.enable(WebGl2RenderingContext::DEPTH_TEST);
        context.pixel_storei(WebGl2RenderingContext::UNPACK_FLIP_Y_WEBGL, 1);
        Renderer {
            properties,
            global_uniform_updater,
        }
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
        gl::set_clear_color(context, &self.properties.clear_color);
        context.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );
        let canvas = web::get_canvas(context).expect("Canvas not found");
        camera
            .borrow_mut()
            .set_aspect_ratio(canvas.client_width() as f32 / canvas.client_height() as f32);
        scene.render(context, camera, self.global_uniform_updater.as_ref())
    }
}

#[derive(Debug, Clone)]
struct DefaultGlobalUniformUpdater;

impl UpdateUniforms for DefaultGlobalUniformUpdater {
    fn update_uniforms(
        &self,
        _context: &WebGl2RenderingContext,
        _program: &crate::gltf::program::Program,
    ) {
    }
}
