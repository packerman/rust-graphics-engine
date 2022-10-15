use std::cell::RefCell;

use web_sys::WebGl2RenderingContext;

use super::{
    camera::Camera, color::Color, gl, math::resolution::Resolution, node::Node,
    render_target::RenderTarget, web,
};

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

pub struct Renderer;

impl Renderer {
    pub const CLEAR_ALL: u32 =
        WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT;
    pub const CLEAR_DEPTH_ONLY: u32 = WebGl2RenderingContext::DEPTH_BUFFER_BIT;

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
        self.render_generic(context, scene, camera, Self::CLEAR_ALL, None);
    }

    pub fn render_clear(
        &self,
        context: &WebGl2RenderingContext,
        scene: &Node,
        camera: &RefCell<Camera>,
        clear_mask: u32,
    ) {
        self.render_generic(context, scene, camera, clear_mask, None)
    }

    pub fn render_to_target(
        &self,
        context: &WebGl2RenderingContext,
        scene: &Node,
        camera: &RefCell<Camera>,
        render_target: Option<&RenderTarget>,
    ) {
        self.render_generic(context, scene, camera, Self::CLEAR_ALL, render_target);
    }

    pub fn render_generic(
        &self,
        context: &WebGl2RenderingContext,
        scene: &Node,
        camera: &RefCell<Camera>,
        clear_mask: u32,
        render_target: Option<&RenderTarget>,
    ) {
        let resolution: Resolution;
        if let Some(render_target) = render_target {
            render_target.bind(context);
            resolution = render_target.resolution();
        } else {
            context.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
            resolution = self::get_canvas_size(context);
        }
        context.clear(clear_mask);
        self.render_with_resolution(context, scene, camera, resolution)
    }

    fn render_with_resolution(
        &self,
        context: &WebGl2RenderingContext,
        scene: &Node,
        camera: &RefCell<Camera>,
        resolution: Resolution,
    ) {
        self::viewport(context, resolution);

        let nodes = scene.descendants();

        for node in nodes.iter() {
            if let Some(camera) = node.camera() {
                camera.borrow_mut().set_aspect_ratio(resolution.ratio());
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

pub fn get_canvas_size(context: &WebGl2RenderingContext) -> Resolution {
    let canvas = web::get_canvas(context).unwrap();
    let (width, height) = web::canvas_size(&canvas);
    Resolution::new(width as i32, height as i32)
}

fn viewport(context: &WebGl2RenderingContext, resolution: Resolution) {
    context.viewport(0, 0, resolution.width, resolution.height)
}
