use anyhow::Result;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::core::application::Application;
use crate::core::attribute::{Attribute, DataType};
use crate::core::color::gray;
use crate::core::gl::{build_program, create_vertex_array, set_clear_color};
use crate::core::input::KeyState;

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
    vertex_count: usize,
}

impl VertexColors {
    #[allow(dead_code)]
    pub fn create(context: &WebGl2RenderingContext) -> Result<Box<dyn Application>> {
        log!("Initializing...");
        set_clear_color(context, &gray());
        let program = build_program(context, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE)?;
        context.line_width(4.0);
        let vao = create_vertex_array(context)?;
        context.bind_vertex_array(Some(&vao));
        let position_data = [
            [0.8_f32, 0.0, 0.0],
            [0.4, 0.6, 0.0],
            [-0.4, 0.6, 0.0],
            [-0.8, 0.0, 0.0],
            [-0.4, -0.6, 0.0],
            [0.4, -0.6, 0.0],
        ];
        let vertex_count = position_data.len();
        let position_attribute =
            Attribute::new_with_data(context, &DataType::VEC3, &position_data)?;
        position_attribute.associate_variable(context, &program, "position")?;

        let color_data = [
            [1.0_f32, 0.0, 0.0],
            [1.0, 0.5, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.5, 0.0, 1.0],
        ];
        let color_attribute = Attribute::new_with_data(context, &DataType::VEC3, &color_data)?;
        color_attribute.associate_variable(context, &program, "vertexColor")?;

        Ok(Box::new(VertexColors {
            program,
            vertex_count,
        }))
    }
}

impl Application for VertexColors {
    fn update(&mut self, key_state: &KeyState) {}

    fn render(&self, context: &WebGl2RenderingContext) {
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        context.use_program(Some(&self.program));
        context.draw_arrays(
            WebGl2RenderingContext::TRIANGLE_FAN,
            0,
            self.vertex_count.try_into().unwrap(),
        );
    }
}
