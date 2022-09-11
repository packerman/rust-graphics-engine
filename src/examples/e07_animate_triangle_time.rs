use anyhow::Result;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::core::{
    application::Application,
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

pub struct AnimateTriangleTime<'a> {
    program: WebGlProgram,
    position: Attribute,
    translation: Uniform<'a>,
    base_color: Uniform<'a>,
    frame: usize,
}

impl AnimateTriangleTime<'_> {
    pub fn create(context: &WebGl2RenderingContext) -> Result<Box<dyn Application>> {
        log!("Initializing...");
        gl::set_clear_color(context, &Color::gray());
        let program = gl::build_program(context, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE)?;
        let vao = gl::create_vertex_array(context)?;
        context.bind_vertex_array(Some(&vao));
        let position_data = [[0.0_f32, 0.2, 0.0], [0.2, -0.2, 0.0], [-0.2, -0.2, 0.0]];
        let position_attribute =
            Attribute::new_with_data(context, AttributeData::from(&position_data))?;
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

        Ok(Box::new(AnimateTriangleTime {
            program,
            position: position_attribute,
            translation,
            base_color,
            frame: 0,
        }))
    }
}

impl Application for AnimateTriangleTime<'_> {
    fn update(&mut self, _key_state: &KeyState) {
        let t = self.frame as f32 / 60.0;
        let translation = self.translation.array3_mut().unwrap();
        translation[0] = 0.75 * t.cos();
        translation[1] = 0.75 * t.sin();

        let color = self.base_color.color_mut().unwrap();
        color[0] = (t.sin() + 1.0) / 2.0;
        color[1] = ((t + 2.1).sin() + 1.0) / 2.0;
        color[2] = ((t + 4.2).sin() + 1.0) / 2.0;
        self.frame += 1;
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
