use anyhow::Result;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlVertexArrayObject};

use crate::core::{
    application::Application,
    attribute::{Attribute, DataType},
    color::black,
    gl::{build_program, create_vertex_array, set_clear_color},
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

pub struct TwoShapes {
    program: WebGlProgram,
    vertex_count_triangle: usize,
    vao_triangle: WebGlVertexArrayObject,
    vertex_count_square: usize,
    vao_square: WebGlVertexArrayObject,
}

impl TwoShapes {
    #[allow(dead_code)]
    pub fn create(context: &WebGl2RenderingContext) -> Result<Box<dyn Application>> {
        log!("Initialized");
        set_clear_color(context, &black());
        let program = build_program(context, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE)?;
        context.line_width(4.0);

        let vao_triangle = create_vertex_array(context)?;
        context.bind_vertex_array(Some(&vao_triangle));
        let position_data_triangle = [[-0.5_f32, 0.8, 0.0], [-0.2, 0.2, 0.0], [-0.8, 0.2, 0.0]];
        let vertex_count_triangle = position_data_triangle.len();
        let position_attribute_triangle =
            Attribute::new_with_data(context, &DataType::VEC3, &position_data_triangle)?;
        position_attribute_triangle.associate_variable(context, &program, "position")?;

        let vao_square = create_vertex_array(context)?;
        context.bind_vertex_array(Some(&vao_square));
        let position_data_square = [
            [0.8_f32, 0.8, 0.0],
            [0.8, 0.2, 0.0],
            [0.2, 0.2, 0.0],
            [0.2, 0.8, 0.0],
        ];
        let vertex_count_square = position_data_square.len();
        let position_attribute_square =
            Attribute::new_with_data(context, &DataType::VEC3, &position_data_square)?;
        position_attribute_square.associate_variable(context, &program, "position")?;

        Ok(Box::new(TwoShapes {
            program,
            vertex_count_triangle,
            vao_triangle,
            vertex_count_square,
            vao_square,
        }))
    }
}

impl Application for TwoShapes {
    fn update(&mut self, _key_state: &KeyState) {}

    fn render(&self, context: &WebGl2RenderingContext) {
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        context.use_program(Some(&self.program));

        context.bind_vertex_array(Some(&self.vao_triangle));
        context.draw_arrays(
            WebGl2RenderingContext::LINE_LOOP,
            0,
            self.vertex_count_triangle.try_into().unwrap(),
        );

        context.bind_vertex_array(Some(&self.vao_square));
        context.draw_arrays(
            WebGl2RenderingContext::LINE_LOOP,
            0,
            self.vertex_count_square.try_into().unwrap(),
        );
    }
}
