use anyhow::Result;
use async_trait::async_trait;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::core::{
    application::{Application, AsyncCreator},
    attribute::{Attribute, AttributeData},
    color::Color,
    gl,
    input::KeyState,
    uniform::{Uniform, UniformData},
};

const VERTEX_SHADER_SOURCE: &str = r##"#version 300 es
in vec3 position;
uniform vec3 translation;
void main()
{
    gl_Position = vec4(position + translation, 1.0);
}
"##;

const FRAGMENT_SHADER_SOURCE: &str = r##"#version 300 es
precision highp float;
uniform vec4 baseColor;
out vec4 fragColor;
void main()
{
    fragColor = baseColor;
}
"##;

pub struct Example {
    program: WebGlProgram,
    position: Attribute,
    translation1: Uniform,
    translation2: Uniform,
    base_color1: Uniform,
    base_color2: Uniform,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Self> {
        gl::set_clear_color(context, &Color::gray());
        let program = gl::build_program(context, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE)?;
        let vao = gl::create_vertex_array(context)?;
        context.bind_vertex_array(Some(&vao));
        let position_data = [[0.0_f32, 0.2, 0.0], [0.2, -0.2, 0.0], [-0.2, -0.2, 0.0]];
        let position_attribute =
            Attribute::new_with_data(context, AttributeData::from(&position_data))?;
        position_attribute.associate_variable(context, &program, "position");

        let translation1 = Uniform::new_with_data(
            context,
            UniformData::from([-0.5_f32, 0.0, 0.0]),
            &program,
            "translation",
        )?;
        let translation2 = Uniform::new_with_data(
            context,
            UniformData::from([0.5_f32, 0.0, 0.0]),
            &program,
            "translation",
        )?;
        let base_color1 = Uniform::new_with_data(
            context,
            UniformData::from(Color::red()),
            &program,
            "baseColor",
        )?;
        let base_color2 = Uniform::new_with_data(
            context,
            UniformData::from(Color::blue()),
            &program,
            "baseColor",
        )?;

        Ok(Example {
            program,
            position: position_attribute,
            translation1,
            translation2,
            base_color1,
            base_color2,
        })
    }
}

impl Application for Example {
    fn update(&mut self, _key_state: &KeyState) {}

    fn render(&self, context: &WebGl2RenderingContext) {
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        context.use_program(Some(&self.program));
        self.translation1.upload_data(context);
        self.base_color1.upload_data(context);
        context.draw_arrays(
            WebGl2RenderingContext::TRIANGLE_FAN,
            0,
            self.position.count(),
        );
        self.translation2.upload_data(context);
        self.base_color2.upload_data(context);
        context.draw_arrays(
            WebGl2RenderingContext::TRIANGLE_FAN,
            0,
            self.position.count(),
        );
    }
}
