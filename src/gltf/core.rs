use web_sys::WebGl2RenderingContext;

use crate::{
    base::{
        input::KeyState,
        util::shared_ref::{self, SharedRef},
    },
    core::{
        camera::Camera,
        node::Node,
        program::{Program, UpdateProgramUniforms, UpdateUniform},
        renderer::Renderer,
        scene::Scene,
    },
    extras::camera_controller::CameraController,
};

use super::user::light_controller::LightController;

#[derive(Debug)]
pub struct Root {
    cameras: Vec<SharedRef<Camera>>,
    scenes: Vec<Scene>,
    scene: Option<usize>,
    renderer: Renderer,
    current_scene_index: Option<usize>,
    current_camera_index: Option<usize>,
    camera_controller: Option<CameraController>,
    light_controller: SharedRef<LightController>,
}

impl Root {
    pub fn initialize(
        context: &WebGl2RenderingContext,
        mut cameras: Vec<SharedRef<Camera>>,
        mut scenes: Vec<Scene>,
        scene: Option<usize>,
    ) -> Self {
        scenes
            .iter_mut()
            .for_each(|scene| Self::ensure_camera_for_scene(scene, &mut cameras));
        debug!(
            "Scene depths: {:#?}",
            scenes.iter().map(|scene| scene.depth()).collect::<Vec<_>>()
        );
        let light_controller = shared_ref::strong(LightController::new(Default::default()));
        let renderer = Renderer::initialize(
            context,
            Default::default(),
            Box::new(GlobalUniformUpdater::new(light_controller.clone())),
        );
        let mut root = Self {
            cameras,
            scenes,
            scene,
            renderer,
            current_scene_index: None,
            current_camera_index: None,
            camera_controller: None,
            light_controller,
        };
        root.set_default_scene();
        root
    }

    pub fn set_default_scene(&mut self) {
        self.set_scene_by_index(self.scene)
    }

    pub fn set_camera_by_index(&mut self, camera_index: Option<usize>) {
        self.current_camera_index = camera_index;
        self.camera_controller =
            camera_index.and_then(|index| CameraController::make_for_camera(&self.cameras[index]))
    }

    pub fn set_scene_by_index(&mut self, scene_index: Option<usize>) {
        let camera_index = scene_index.and_then(|index| self.find_camera_for_scene(index));
        self.set_scene_and_camera_by_index(scene_index, camera_index)
    }

    pub fn set_scene_and_camera_by_index(
        &mut self,
        scene_index: Option<usize>,
        camera_index: Option<usize>,
    ) {
        self.current_scene_index = scene_index;
        self.set_camera_by_index(camera_index);
    }

    pub fn update(&self, key_state: &KeyState) {
        self.light_controller.borrow_mut().update(key_state);
        if let Some(camera_controller) = &self.camera_controller {
            camera_controller.update(key_state);
        }
    }

    pub fn render(&self, context: &WebGl2RenderingContext) {
        if let Some(scene_index) = self.current_scene_index {
            if let Some(camera_index) = self.current_camera_index {
                self.renderer.render(
                    context,
                    &self.scenes[scene_index],
                    &self.cameras[camera_index],
                )
            }
        }
    }

    fn find_camera_for_scene(&self, scene_index: usize) -> Option<usize> {
        self.scenes.get(scene_index).and_then(|scene| {
            self.cameras
                .iter()
                .position(|camera| scene.contains_camera(camera))
        })
    }

    fn ensure_camera_for_scene(scene: &mut Scene, cameras: &mut Vec<SharedRef<Camera>>) {
        if !scene.has_some_camera() {
            let camera = Self::default_camera();
            let node = Node::with_camera_and_name(camera.clone(), "Default camera");
            scene.add_root_node(node);
            cameras.push(camera);
        }
    }

    fn default_camera() -> SharedRef<Camera> {
        shared_ref::strong(Camera::perspective(
            1.0,
            60_f32.to_radians(),
            0.01,
            Some(100.0),
            Some("Default camera".into()),
        ))
    }
}

#[derive(Debug, Clone)]
struct GlobalUniformUpdater {
    light_controller: SharedRef<LightController>,
}

impl GlobalUniformUpdater {
    fn new(light_controller: SharedRef<LightController>) -> Self {
        Self { light_controller }
    }
}

impl UpdateProgramUniforms for GlobalUniformUpdater {
    fn update_program_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {
        let light_direction = self.light_controller.borrow().get_light_direction();
        light_direction.update_uniform(context, "u_Light", program);
    }
}
