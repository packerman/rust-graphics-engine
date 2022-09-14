use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::core::{
    application::{Application, AsyncCreator},
    color::Color,
    gl,
    input::KeyState,
};

pub struct TestExample;

#[async_trait(?Send)]
impl AsyncCreator for TestExample {
    async fn create(context: &WebGl2RenderingContext) -> Result<Self> {
        gl::set_clear_color(context, &Color::black());
        Ok(TestExample)
    }
}

impl Application for TestExample {
    fn update(&mut self, _key_state: &KeyState) {}

    fn render(&self, context: &WebGl2RenderingContext) {
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }
}
