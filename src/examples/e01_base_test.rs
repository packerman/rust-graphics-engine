use web_sys::WebGl2RenderingContext;

use crate::core::{application::Application, color, gl::set_clear_color};

pub struct TestApp;

impl TestApp {
    #[allow(dead_code)]
    pub fn create(context: &WebGl2RenderingContext) -> Box<dyn Application> {
        log!("Initialized");
        set_clear_color(context, &color::black());
        Box::new(TestApp)
    }
}

impl Application for TestApp {
    fn update(&mut self) {}
    fn render(&self, context: &WebGl2RenderingContext) {
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }
}
