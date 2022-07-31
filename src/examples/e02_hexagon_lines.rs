use anyhow::Result;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::core::application::Application;
use crate::core::attribute::{Attribute, DataType};
use crate::core::color::black;
use crate::core::gl::{build_program, create_vertex_array, set_clear_color};

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
    vertex_count: usize,
}

impl HexagonLines {
    #[allow(dead_code)]
    pub fn create(context: &WebGl2RenderingContext) -> Result<Box<dyn Application>> {
        log!("Initialized");
        set_clear_color(context, &black());
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
        Ok(Box::new(HexagonLines {
            program,
            vertex_count,
        }))
    }
}

impl Application for HexagonLines {
    fn update(&mut self) {}
    fn render(&self, context: &WebGl2RenderingContext) {
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        context.use_program(Some(&self.program));
        context.draw_arrays(
            WebGl2RenderingContext::LINE_LOOP,
            0,
            self.vertex_count.try_into().unwrap(),
        );
    }
}
