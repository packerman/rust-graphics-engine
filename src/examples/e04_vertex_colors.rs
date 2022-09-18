use anyhow::Result;
use async_trait::async_trait;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::core::{
    application::{self, Application, AsyncCreator},
    attribute::{Attribute, AttributeData},
    color::Color,
    gl,
    input::KeyState,
};

const VERTEX_SHADER_SOURCE: &str = r##"#version 300 es
in vec3 position;
in vec3 vertexColor;
out vec3 color;
void main()
{
    gl_PointSize = 10.0;
    gl_Position = vec4(position, 1.0);
    color = vertexColor;
}
"##;

const FRAGMENT_SHADER_SOURCE: &str = r##"#version 300 es
precision highp float;
in vec3 color;
out vec4 fragColor;
void main()
{
    fragColor = vec4(color, 1.0);
}
"##;

struct Example {
    program: WebGlProgram,
    position_attribute: Attribute,
    #[allow(dead_code)]
    color_attribute: Attribute,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Self> {
        gl::set_clear_color(context, &Color::gray());
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
        let position_attribute =
            Attribute::new_with_data(context, AttributeData::from(&position_data))?;
        position_attribute.associate_variable(context, &program, "position");

        let color_data: [[f32; 3]; 6] = [
            Color::red().into(),
            Color::dark_orange().into(),
            Color::yellow().into(),
            Color::lime().into(),
            Color::blue().into(),
            Color::blue_violet().into(),
        ];
        let color_attribute = Attribute::new_with_data(context, AttributeData::from(&color_data))?;
        color_attribute.associate_variable(context, &program, "vertexColor");

        Ok(Example {
            program,
            position_attribute,
            color_attribute,
        })
    }
}

impl Application for Example {
    fn update(&mut self, _key_state: &KeyState) {}

    fn render(&self, context: &WebGl2RenderingContext) {
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        context.use_program(Some(&self.program));
        context.draw_arrays(
            WebGl2RenderingContext::TRIANGLE_FAN,
            0,
            self.position_attribute.count(),
        );
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
