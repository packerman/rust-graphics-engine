use web_sys::WebGl2RenderingContext;

use self::{
    camera::Camera,
    scene::{Node, Scene},
};

use super::util::shared_ref::SharedRef;

pub mod camera;
pub mod geometry;
pub mod material;
pub mod scene;
pub mod storage;

#[derive(Debug, Clone)]
pub struct Root {
    cameras: Vec<SharedRef<Camera>>,
    scenes: Vec<Scene>,
    scene: Option<usize>,
}

impl Root {
    pub fn new(
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
        Self {
            cameras,
            scenes,
            scene,
        }
    }

    pub fn render(&self, context: &WebGl2RenderingContext) {
        if let Some(scene) = self.scene {
            self.render_with_index(context, scene);
        }
    }

    pub fn render_with_index(&self, context: &WebGl2RenderingContext, scene_index: usize) {
        let scene = &self.scenes[scene_index];
        if let Some(camera_index) = self.find_camera_for_scene(scene) {
            self.render_with_index_and_camera(context, scene_index, camera_index);
        }
    }

    pub fn render_with_index_and_camera(
        &self,
        context: &WebGl2RenderingContext,
        scene_index: usize,
        camera_index: usize,
    ) {
        let camera = &self.cameras[camera_index];
        self.scenes[scene_index].render(context, camera);
    }

    fn find_camera_for_scene(&self, scene: &Scene) -> Option<usize> {
        self.cameras
            .iter()
            .position(|camera| scene.contains_camera(camera))
    }

    fn ensure_camera_for_scene(scene: &mut Scene, cameras: &mut Vec<SharedRef<Camera>>) {
        if !scene.has_some_camera() {
            let camera = Self::default_camera();
            let node = Node::new(
                glm::translation(&glm::vec3(0.5, 0.5, 3.0)),
                None,
                camera.clone().into(),
                Some("Default camera".into()),
            );
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
