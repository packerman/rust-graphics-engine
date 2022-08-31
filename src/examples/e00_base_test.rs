use anyhow::Result;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

use crate::core::{application::Application, color::Color, gl, input::KeyState};

pub struct TestApp;

impl TestApp {
    pub fn create(
        context: &WebGl2RenderingContext,
        _canvas: &HtmlCanvasElement,
    ) -> Result<Box<dyn Application>> {
        log!("Initialized");
        gl::set_clear_color(context, &Color::black());
        Ok(Box::new(TestApp))
    }
}

impl Application for TestApp {
    fn update(&mut self, _key_state: &KeyState) {}

    fn render(&self, context: &WebGl2RenderingContext) {
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }
}
