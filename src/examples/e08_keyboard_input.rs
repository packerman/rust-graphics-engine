use anyhow::Result;
use async_trait::async_trait;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::core::{
    application::{self, Application, AsyncCreator},
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

struct Example {
    program: WebGlProgram,
    position: Attribute,
    translation: Uniform,
    base_color: Uniform,
}

const SPEED: f32 = 0.5;

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        gl::set_clear_color(context, &Color::gray());
        let program = gl::build_program(context, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE)?;
        let vao = gl::create_vertex_array(context)?;
        context.bind_vertex_array(Some(&vao));
        let position_data = [[0.0_f32, 0.2, 0.0], [0.2, -0.2, 0.0], [-0.2, -0.2, 0.0]];
        let position_attribute =
            Attribute::new_with_data(context, AttributeData::from(&position_data))?;
        position_attribute.associate_variable(context, &program, "position");

        let translation = Uniform::new_with_data(
            context,
            UniformData::from(glm::vec3(-0.5_f32, 0.0, 0.0)),
            &program,
            "translation",
        )?;
        let base_color = Uniform::new_with_data(
            context,
            UniformData::from(Color::red()),
            &program,
            "baseColor",
        )?;

        Ok(Box::new(Example {
            program,
            position: position_attribute,
            translation,
            base_color,
        }))
    }
}

impl Application for Example {
    fn update(&mut self, key_state: &KeyState) {
        let distance = SPEED / 60.0;

        if let Ok(mut translation) = self.translation.vec3_mut() {
            if key_state.is_pressed("ArrowLeft") {
                translation[0] -= distance;
            }
            if key_state.is_pressed("ArrowRight") {
                translation[0] += distance;
            }
            if key_state.is_pressed("ArrowDown") {
                translation[1] -= distance;
            }
            if key_state.is_pressed("ArrowUp") {
                translation[1] += distance;
            }
        }
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        context.use_program(Some(&self.program));
        self.translation.upload_data(context);
        self.base_color.upload_data(context);
        context.draw_arrays(
            WebGl2RenderingContext::TRIANGLE_FAN,
            0,
            self.position.count(),
        );
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
