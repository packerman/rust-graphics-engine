use anyhow::Result;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::core::{
    application::Application,
    attribute::{Attribute, AttributeFactory},
    color::Color,
    gl,
    input::KeyState,
};

const VERTEX_SHADER_SOURCE: &str = r##"#version 300 es
in vec4 position;
void main()
{
    gl_Position = vec4(position.x, position.y, position.z , 1.0);
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

pub struct HexagonLines {
    program: WebGlProgram,
    attribute: Attribute,
}

impl HexagonLines {
    pub fn create(context: &WebGl2RenderingContext) -> Result<Box<dyn Application>> {
        log!("Initialized");
        gl::set_clear_color(context, &Color::black());
        let program = gl::build_program(context, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE)?;
        context.line_width(4.0);
        let vao = gl::create_vertex_array(context)?;
        context.bind_vertex_array(Some(&vao));
        let position_data = [
            [0.8_f32, 0.0, 0.0],
            [0.4, 0.6, 0.0],
            [-0.4, 0.6, 0.0],
            [-0.8, 0.0, 0.0],
            [-0.4, -0.6, 0.0],
            [0.4, -0.6, 0.0],
        ];
        let factory = AttributeFactory::new(context);
        let position_attribute = factory.with_array(&position_data)?;
        position_attribute.associate_variable(context, &program, "position")?;
        Ok(Box::new(HexagonLines {
            program,
            attribute: position_attribute,
        }))
    }
}

impl Application for HexagonLines {
    fn update(&mut self, _key_state: &KeyState) {}

    fn render(&self, context: &WebGl2RenderingContext) {
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        context.use_program(Some(&self.program));
        context.draw_arrays(
            WebGl2RenderingContext::LINE_LOOP,
            0,
            self.attribute.vertex_count.try_into().unwrap(),
        );
    }
}
