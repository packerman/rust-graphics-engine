use web_sys::WebGl2RenderingContext;

use crate::core::input::KeyState;

use self::{
    camera::Camera,
    renderer::Renderer,
    scene::{Node, Scene},
};

use super::{user::MovementRig, util::shared_ref::SharedRef};

pub mod camera;
pub mod geometry;
pub mod material;
pub mod renderer;
pub mod scene;
pub mod storage;

#[derive(Debug, Clone)]
pub struct Root {
    cameras: Vec<SharedRef<Camera>>,
    scenes: Vec<Scene>,
    scene: Option<usize>,
    renderer: Renderer,
    current_scene_index: Option<usize>,
    current_camera_index: Option<usize>,
    movement_rig: Option<MovementRig>,
}

impl Root {
    pub fn initialize(
        mut cameras: Vec<SharedRef<Camera>>,
        mut scenes: Vec<Scene>,
        scene: Option<usize>,
        renderer: Renderer,
    ) -> Self {
        scenes
            .iter_mut()
            .for_each(|scene| Self::ensure_camera_for_scene(scene, &mut cameras));
        debug!(
            "Scene depths: {:#?}",
            scenes.iter().map(|scene| scene.depth()).collect::<Vec<_>>()
        );
        let mut root = Self {
            cameras,
            scenes,
            scene,
            renderer,
            current_scene_index: None,
            current_camera_index: None,
            movement_rig: None,
        };
        root.set_default_scene();
        root
    }

    pub fn set_default_scene(&mut self) {
        self.set_scene_by_index(self.scene)
    }

    pub fn set_camera_by_index(&mut self, camera_index: Option<usize>) {
        self.current_camera_index = camera_index;
        self.movement_rig =
            camera_index.and_then(|index| MovementRig::make_for_camera(&self.cameras[index]))
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
        if let Some(movement_rig) = &self.movement_rig {
            movement_rig.update(key_state);
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
        Camera::perspective(
            1.0,
            60_f32.to_radians(),
            0.01,
            Some(100.0),
            Some("Default camera".into()),
        )
        .into()
    }
}
