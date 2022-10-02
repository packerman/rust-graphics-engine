use std::cell::RefCell;

use web_sys::WebGl2RenderingContext;

use super::{camera::Camera, color::Color, gl, node::Node, web};

pub struct RendererOptions {
    pub clear_color: Color,
    pub blending: bool,
    pub flip_y: bool,
}

impl Default for RendererOptions {
    fn default() -> Self {
        Self {
            clear_color: Color::black(),
            blending: true,
            flip_y: true,
        }
    }
}

pub struct ClearBuffers {
    pub color: bool,
    pub depth: bool,
}

impl ClearBuffers {
    pub const ALL: Self = Self {
        color: true,
        depth: true,
    };

    pub const DEPTH_ONLY: Self = Self {
        color: false,
        depth: true,
    };

    fn call(&self, context: &WebGl2RenderingContext) {
        if self.color {
            context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        }
        if self.depth {
            context.clear(WebGl2RenderingContext::DEPTH_BUFFER_BIT);
        }
    }
}

pub struct Renderer;

impl Renderer {
    pub fn new(context: &WebGl2RenderingContext, options: RendererOptions) -> Self {
        context.enable(WebGl2RenderingContext::DEPTH_TEST);
        gl::set_clear_color(context, &options.clear_color);

        if options.blending {
            context.enable(WebGl2RenderingContext::BLEND);
            context.blend_func(
                WebGl2RenderingContext::SRC_ALPHA,
                WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA,
            );
        }

        if options.flip_y {
            context.pixel_storei(WebGl2RenderingContext::UNPACK_FLIP_Y_WEBGL, 1);
        }

        Self
    }

    pub fn render(&self, context: &WebGl2RenderingContext, scene: &Node, camera: &RefCell<Camera>) {
        self.render_clear(context, scene, camera, ClearBuffers::ALL);
    }

    pub fn render_clear(
        &self,
        context: &WebGl2RenderingContext,
        scene: &Node,
        camera: &RefCell<Camera>,
        clear_buffers: ClearBuffers,
    ) {
        clear_buffers.call(context);
        let canvas = web::get_canvas(context).unwrap();
        let (width, height) = web::canvas_size(&canvas);
        context.viewport(0, 0, width as i32, height as i32);
        let nodes = scene.descendants();

        for node in nodes.iter() {
            if let Some(camera) = node.camera() {
                camera.borrow_mut().set_aspect_ratio(width, height);
                camera.borrow_mut().update_view_matrix(&node.world_matrix());
            }
        }

        let camera = &camera.borrow();
        for node in nodes.iter() {
            if let Some(mesh) = node.mesh() {
                mesh.render(context, camera, node.world_matrix())
            }
        }
    }
}
