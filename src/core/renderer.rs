use std::cell::RefCell;

use web_sys::WebGl2RenderingContext;

use super::{camera::Camera, color::Color, gl, node::Node, render_target::RenderTarget, web};

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
        self.render_generic(context, scene, camera, ClearBuffers::ALL, None);
    }

    pub fn render_clear(
        &self,
        context: &WebGl2RenderingContext,
        scene: &Node,
        camera: &RefCell<Camera>,
        clear_buffers: ClearBuffers,
    ) {
        self.render_generic(context, scene, camera, clear_buffers, None)
    }

    pub fn render_to_target(
        &self,
        context: &WebGl2RenderingContext,
        scene: &Node,
        camera: &RefCell<Camera>,
        render_target: &RenderTarget,
    ) {
        self.render_generic(
            context,
            scene,
            camera,
            ClearBuffers::ALL,
            Some(render_target),
        );
    }

    pub fn render_generic(
        &self,
        context: &WebGl2RenderingContext,
        scene: &Node,
        camera: &RefCell<Camera>,
        clear_buffers: ClearBuffers,
        render_target: Option<&RenderTarget>,
    ) {
        clear_buffers.call(context);
        let resolution: (i32, i32);
        if let Some(render_target) = render_target {
            render_target.bind(context);
            resolution = render_target.size();
        } else {
            context.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
            resolution = self::get_canvas_size(context);
        }
        self.render_with_resolution(context, scene, camera, resolution)
    }

    fn render_with_resolution(
        &self,
        context: &WebGl2RenderingContext,
        scene: &Node,
        camera: &RefCell<Camera>,
        resolution: (i32, i32),
    ) {
        self::viewport(context, resolution);

        let nodes = scene.descendants();

        for node in nodes.iter() {
            if let Some(camera) = node.camera() {
                camera.borrow_mut().set_aspect_ratio(resolution);
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

fn get_canvas_size(context: &WebGl2RenderingContext) -> (i32, i32) {
    let canvas = web::get_canvas(context).unwrap();
    let (width, height) = web::canvas_size(&canvas);
    (width as i32, height as i32)
}

fn viewport(context: &WebGl2RenderingContext, size: (i32, i32)) {
    context.viewport(0, 0, size.0, size.1)
}
