use web_sys::WebGl2RenderingContext;

use self::scene::Scene;

pub mod camera;
pub mod geometry;
pub mod scene;
pub mod storage;

#[derive(Debug, Clone)]
pub struct Root {
    scenes: Vec<Scene>,
    scene: Option<usize>,
}

impl Root {
    pub fn new(scenes: Vec<Scene>, scene: Option<usize>) -> Self {
        Self { scenes, scene }
    }

    pub fn render(&self, context: &WebGl2RenderingContext) {
        if let Some(scene) = self.scene {
            self.scenes[scene].render(context);
        }
    }
}
