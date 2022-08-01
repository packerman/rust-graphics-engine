use anyhow::Result;
use web_sys::WebGl2RenderingContext;

use crate::core::application::Application;
use crate::core::color::black;
use crate::core::gl::set_clear_color;
use crate::core::input::KeyState;

pub struct TestApp;

impl TestApp {
    #[allow(dead_code)]
    pub fn create(context: &WebGl2RenderingContext) -> Result<Box<dyn Application>> {
        log!("Initialized");
        set_clear_color(context, &black());
        Ok(Box::new(TestApp))
    }
}

impl Application for TestApp {
    fn update(&mut self, key_state: &KeyState) {}

    fn render(&self, context: &WebGl2RenderingContext) {
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }
}
