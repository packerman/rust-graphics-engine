use std::{cell::RefCell, rc::Rc};

use web_sys::WebGl2RenderingContext;

use crate::{
    base::{
        color::{self, Color},
        gl,
        math::resolution::Resolution,
        util::{level::Level, shared_ref::SharedRef},
        web,
    },
    core::{
        camera::Camera, material, mesh::Mesh, node::Node, program::UpdateProgramUniforms,
        scene::Scene,
    },
};

use super::{
    light::{Light, Lights},
    render_target::RenderTarget,
    shadow::Shadow,
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
            clear_color: color::gray(),
            blending: true,
            flip_y: true,
            light_count: 4,
        }
    }
}

#[derive(Debug)]
pub struct Renderer {
    default_light: RefCell<Light>,
    light_count: usize,
    shadow: Option<Shadow>,
    clear_color: Color,
    global_uniform_updater: Box<dyn UpdateProgramUniforms>,
}

impl Renderer {
    pub const CLEAR_ALL: u32 =
        WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT;
    pub const CLEAR_DEPTH_ONLY: u32 = WebGl2RenderingContext::DEPTH_BUFFER_BIT;

    pub fn initialize(
        context: &WebGl2RenderingContext,
        options: RendererOptions,
        shadow: Option<Shadow>,
    ) -> Self {
        context.enable(WebGl2RenderingContext::DEPTH_TEST);

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

        let default_light = RefCell::new(Light::default());

        Self {
            default_light,
            light_count: options.light_count,
            shadow,
            clear_color: options.clear_color,
            global_uniform_updater: material::default_uniform_updater(),
        }
    }

    pub fn shadow(&self) -> Option<&Shadow> {
        self.shadow.as_ref()
    }

    pub fn render(
        &self,
        context: &WebGl2RenderingContext,
        scene: &Scene,
        camera: &RefCell<Camera>,
    ) {
        self.render_generic(
            context,
            scene,
            camera,
            Self::CLEAR_ALL,
            None,
            &Lights::new(),
        );
    }

    pub fn render_with_lights(
        &self,
        context: &WebGl2RenderingContext,
        scene: &Scene,
        camera: &RefCell<Camera>,
        lights: &Lights,
    ) {
        self.render_generic(context, scene, camera, Self::CLEAR_ALL, None, lights);
    }

    pub fn render_clear(
        &self,
        context: &WebGl2RenderingContext,
        scene: &Scene,
        camera: &RefCell<Camera>,
        clear_mask: u32,
        lights: &Lights,
    ) {
        self.render_generic(context, scene, camera, clear_mask, None, lights)
    }

    pub fn render_to_target(
        &self,
        context: &WebGl2RenderingContext,
        scene: &Scene,
        camera: &RefCell<Camera>,
        render_target: Option<&RenderTarget>,
        lights: &Lights,
    ) {
        self.render_generic(
            context,
            scene,
            camera,
            Self::CLEAR_ALL,
            render_target,
            lights,
        );
    }

    pub fn render_generic(
        &self,
        context: &WebGl2RenderingContext,
        scene: &Scene,
        camera: &RefCell<Camera>,
        clear_mask: u32,
        render_target: Option<&RenderTarget>,
        lights: &Lights,
    ) {
        let nodes = scene.all_nodes();

        let resolution = self::get_resolution(context, render_target);
        lights.update();
        camera
            .borrow_mut()
            .set_aspect_ratio(resolution.aspect_ratio());

        let meshes = Self::filter_meshes(&nodes);
        self.shadow_pass(context, &meshes);

        let camera = &camera.borrow();

        self::bind_render_target(context, render_target);
        gl::set_clear_color(context, &self.clear_color);
        context.clear(clear_mask);
        self::viewport(context, resolution);

        meshes.into_iter().for_each(|(mesh, node)| {
            Self::update_lights(context, &mesh, &lights);
            self.update_shadow(context, &mesh);
            mesh.update_uniform(
                context,
                "viewPosition",
                &camera.world_position(),
                Level::Ignore,
            );
            mesh.render(
                context,
                &node.as_ref().borrow(),
                &camera.matrix(),
                self.global_uniform_updater.as_ref(),
            );
        });
    }

    fn shadow_pass(
        &self,
        context: &WebGl2RenderingContext,
        meshes: &[(Rc<Mesh>, &SharedRef<Node>)],
    ) {
        if let Some(shadow) = self.shadow() {
            shadow.bind(context);
            context.clear_color(1.0, 0.0, 0.0, 1.0);
            context.clear(
                WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
            );
            let material = shadow.material();
            meshes.iter().for_each(|(mesh, node)| {
                mesh.render_triangle_based(
                    context,
                    &node.as_ref().borrow(),
                    self.global_uniform_updater.as_ref(),
                    material,
                );
            });
        }
    }

    fn update_shadow(&self, context: &WebGl2RenderingContext, mesh: &Mesh) {
        if let Some(shadow) = self.shadow() {
            mesh.update_uniform(context, "shadow0", shadow, Level::Ignore);
        }
    }

    fn update_lights(context: &WebGl2RenderingContext, mesh: &Mesh, lights: &Lights) {
        if mesh.has_uniform("light0") {
            lights.for_each_light_indexed(|(i, light)| {
                mesh.update_uniform(
                    context,
                    &format!("light{}", i),
                    &*light.borrow(),
                    Level::Ignore,
                );
            });
        }
    }

    fn filter_meshes(nodes: &[SharedRef<Node>]) -> Vec<(Rc<Mesh>, &SharedRef<Node>)> {
        nodes
            .iter()
            .filter_map(|node| node.borrow().mesh().map(|mesh| (Rc::clone(mesh), node)))
            .collect()
    }
}

pub fn get_canvas_resolution(context: &WebGl2RenderingContext) -> Resolution {
    let canvas = web::get_canvas(context).unwrap();
    let (width, height) = web::canvas_size(&canvas);
    Resolution::new(width as i32, height as i32)
}

fn get_resolution(
    context: &WebGl2RenderingContext,
    render_target: Option<&RenderTarget>,
) -> Resolution {
    if let Some(render_target) = render_target {
        render_target.resolution()
    } else {
        self::get_canvas_resolution(context)
    }
}

fn bind_render_target(context: &WebGl2RenderingContext, render_target: Option<&RenderTarget>) {
    if let Some(render_target) = render_target {
        render_target.bind(context);
    } else {
        context.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
    }
}

fn viewport(context: &WebGl2RenderingContext, resolution: Resolution) {
    context.viewport(0, 0, resolution.width, resolution.height)
}
