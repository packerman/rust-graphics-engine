use anyhow::Result;
use async_trait::async_trait;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlVertexArrayObject};

use crate::core::{
    application::{Application, AsyncCreator},
    attribute::{Attribute, AttributeData},
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

pub struct TwoShapesExample {
    program: WebGlProgram,
    triangle_position: Attribute,
    vao_triangle: WebGlVertexArrayObject,
    square_position: Attribute,
    vao_square: WebGlVertexArrayObject,
}

#[async_trait(?Send)]
impl AsyncCreator for TwoShapesExample {
    async fn create(context: &WebGl2RenderingContext) -> Result<Self> {
        gl::set_clear_color(context, &Color::black());
        let program = gl::build_program(context, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE)?;
        context.line_width(4.0);

        let vao_triangle = gl::create_vertex_array(context)?;
        context.bind_vertex_array(Some(&vao_triangle));
        let position_data_triangle = [[-0.5_f32, 0.8, 0.0], [-0.2, 0.2, 0.0], [-0.8, 0.2, 0.0]];
        let position_attribute_triangle =
            Attribute::new_with_data(context, AttributeData::from(&position_data_triangle))?;
        position_attribute_triangle.associate_variable(context, &program, "position");

        let vao_square = gl::create_vertex_array(context)?;
        context.bind_vertex_array(Some(&vao_square));
        let position_data_square = [
            [0.8_f32, 0.8, 0.0],
            [0.8, 0.2, 0.0],
            [0.2, 0.2, 0.0],
            [0.2, 0.8, 0.0],
        ];
        let position_attribute_square =
            Attribute::new_with_data(context, AttributeData::from(&position_data_square))?;
        position_attribute_square.associate_variable(context, &program, "position");

        Ok(TwoShapesExample {
            program,
            triangle_position: position_attribute_triangle,
            vao_triangle,
            square_position: position_attribute_square,
            vao_square,
        })
    }
}

impl Application for TwoShapesExample {
    fn update(&mut self, _key_state: &KeyState) {}

    fn render(&self, context: &WebGl2RenderingContext) {
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        context.use_program(Some(&self.program));

        context.bind_vertex_array(Some(&self.vao_triangle));
        context.draw_arrays(
            WebGl2RenderingContext::LINE_LOOP,
            0,
            self.triangle_position.count(),
        );

        context.bind_vertex_array(Some(&self.vao_square));
        context.draw_arrays(
            WebGl2RenderingContext::LINE_LOOP,
            0,
            self.square_position.count(),
        );
    }
}
