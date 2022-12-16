use std::{cell::RefCell, rc::Rc};

use web_sys::WebGl2RenderingContext;

use self::{camera::Camera, scene::Scene};

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
    pub fn new(cameras: Vec<SharedRef<Camera>>, scenes: Vec<Scene>, scene: Option<usize>) -> Self {
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
}
