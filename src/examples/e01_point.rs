use anyhow::{anyhow, Result};
use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::core::{
    application::Application,
    color::black,
    gl::{build_program, set_clear_color},
    input::KeyState,
};

const VERTEX_SHADER_SOURCE: &str = r##"#version 300 es
void main()
{
    gl_PointSize = 10.0;
    gl_Position = vec4(0.0, 0.0, 0.0, 1.0);
}
"##;

const FRAGMENT_SHADER_SOURCE: &str = r##"#version 300 es

precision highp float;
out vec4 fragColor;
void main()
{
    fragColor = vec4(1.0, 1.0, 0.0, 1.0);
}
"##;

pub struct PointApp {
    program: WebGlProgram,
}

impl PointApp {
    pub fn create(context: &WebGl2RenderingContext) -> Result<Box<dyn Application>> {
        set_clear_color(context, &black());
        let program = build_program(context, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE)?;
        let vao = context
            .create_vertex_array()
            .ok_or_else(|| anyhow!("Cannot create vertex array object"))?;
        context.bind_vertex_array(Some(&vao));
        Ok(Box::new(PointApp { program }))
    }
}

impl Application for PointApp {
    fn update(&mut self, _key_state: &KeyState) {}

    fn render(&self, context: &WebGl2RenderingContext) {
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        context.use_program(Some(&self.program));
        context.draw_arrays(WebGl2RenderingContext::POINTS, 0, 1);
    }
}
