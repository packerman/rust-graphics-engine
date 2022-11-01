use std::{cell::RefCell, rc::Rc};

use web_sys::WebGl2RenderingContext;

use crate::light::Light;

use super::{
    camera::Camera, color::Color, gl, math::resolution::Resolution, mesh::Mesh, node::Node,
    render_target::RenderTarget, uniform::UpdateUniform, web,
};

pub struct RendererOptions {
    pub clear_color: Color,
    pub blending: bool,
    pub flip_y: bool,
    pub light_count: usize,
}

impl Default for RendererOptions {
    fn default() -> Self {
        Self {
            clear_color: Color::black(),
            blending: true,
            flip_y: true,
            light_count: 4,
        }
    }
}

pub struct Renderer {
    default_node: Rc<Node>,
    default_light: Light,
    light_count: usize,
}

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

        let default_light = Light::default();
        let default_node = Node::new_group();

        Self {
            default_node,
            default_light,
            light_count: options.light_count,
        }
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

        filter_cameras(&nodes).for_each(|(camera, node)| {
            camera.borrow_mut().set_aspect_ratio(resolution.ratio());
            camera.borrow_mut().update_world_matrix(node.world_matrix());
        });

        let lights = self.filter_lights(&nodes);

        let camera = &camera.borrow();
        filter_meshes(&nodes).for_each(|(mesh, node)| {
            if mesh.material().has_uniform("light0") {
                lights.iter().enumerate().for_each(|(i, (light, _))| {
                    if let Some(uniform) = mesh.material().uniform(&format!("light{}", i)) {
                        light.update_uniform(uniform);
                    }
                });
            }
            if let Some(mut view_position) = mesh.material().vec3_mut("viewPosition") {
                *view_position = camera.world_position();
            }
            mesh.render(context, camera, node.world_matrix());
        });
    }

    fn filter_lights<'a>(&'a self, nodes: &'a [Rc<Node>]) -> Vec<(&Light, &Rc<Node>)> {
        let mut lights: Vec<_> = nodes
            .iter()
            .filter_map(|node| node.light().map(|light| (light, node)))
            .collect();
        lights.resize_with(self.light_count, || {
            (&self.default_light, &self.default_node)
        });
        lights
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

fn filter_cameras(nodes: &[Rc<Node>]) -> impl Iterator<Item = (&RefCell<Camera>, &Rc<Node>)> {
    nodes
        .iter()
        .filter_map(|node| node.camera().map(|camera| (camera, node)))
}

fn filter_meshes(nodes: &[Rc<Node>]) -> impl Iterator<Item = (&Mesh, &Rc<Node>)> {
    nodes
        .iter()
        .filter_map(|node| node.mesh().map(|mesh| (mesh, node)))
}
