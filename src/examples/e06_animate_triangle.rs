use anyhow::Result;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram};

use crate::core::{
    application::Application,
    attribute::Attribute,
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

pub struct AnimateTriangle {
    program: WebGlProgram,
    position: Attribute,
    translation: Uniform,
    base_color: Uniform,
}

impl AnimateTriangle {
    pub fn create(
        context: &WebGl2RenderingContext,
        _canvas: &HtmlCanvasElement,
    ) -> Result<Box<dyn Application>> {
        log!("Initializing...");
        gl::set_clear_color(context, &Color::gray());
        let program = gl::build_program(context, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE)?;
        let vao = gl::create_vertex_array(context)?;
        context.bind_vertex_array(Some(&vao));
        let position_data = [[0.0_f32, 0.2, 0.0], [0.2, -0.2, 0.0], [-0.2, -0.2, 0.0]];
        let position_attribute = Attribute::with_array(context, &position_data)?;
        position_attribute.associate_variable(context, &program, "position")?;

        let translation = Uniform::new_with_data(
            context,
            UniformData::from([-0.5_f32, 0.0, 0.0]),
            &program,
            "translation",
        )?;
        let base_color = Uniform::new_with_data(
            context,
            UniformData::from(Color::red()),
            &program,
            "baseColor",
        )?;

        Ok(Box::new(AnimateTriangle {
            program,
            position: position_attribute,
            translation,
            base_color,
        }))
    }
}

impl Application for AnimateTriangle {
    fn update(&mut self, _key_state: &KeyState) {
        let translation = self.translation.array3_mut().unwrap();
        translation[0] += 0.01;
        if translation[0] > 1.2 {
            translation[0] = -1.2;
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
            self.position.vertex_count.try_into().unwrap(),
        );
        context.draw_arrays(
            WebGl2RenderingContext::TRIANGLE_FAN,
            0,
            self.position.vertex_count.try_into().unwrap(),
        );
    }
}
