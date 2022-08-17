use anyhow::Result;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::core::{application::Application, attribute::Attribute, color, gl, input::KeyState};

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

pub struct VertexColors {
    program: WebGlProgram,
    position_attribute: Attribute,
    color_attribute: Attribute,
}

impl VertexColors {
    pub fn create(context: &WebGl2RenderingContext) -> Result<Box<dyn Application>> {
        log!("Initializing...");
        gl::set_clear_color(context, &color::gray());
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
        let position_attribute = Attribute::from_array(context, &position_data)?;
        position_attribute.associate_variable(context, &program, "position")?;

        let color_data = [
            color::to_array3(&color::red()),
            color::to_array3(&color::dark_orange()),
            color::to_array3(&color::yellow()),
            color::to_array3(&color::lime()),
            color::to_array3(&color::blue()),
            color::to_array3(&color::blue_violet()),
        ];
        let color_attribute = Attribute::from_array(context, &color_data)?;
        color_attribute.associate_variable(context, &program, "vertexColor")?;

        Ok(Box::new(VertexColors {
            program,
            position_attribute,
            color_attribute,
        }))
    }
}

impl Application for VertexColors {
    fn update(&mut self, _key_state: &KeyState) {}

    fn render(&self, context: &WebGl2RenderingContext) {
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        context.use_program(Some(&self.program));
        context.draw_arrays(
            WebGl2RenderingContext::TRIANGLE_FAN,
            0,
            self.position_attribute.vertex_count.try_into().unwrap(),
        );
    }
}
