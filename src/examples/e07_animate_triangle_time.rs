use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::core::{
    application::Application,
    attribute::{Attribute, AttributeFactory},
    color::Color,
    gl,
    input::KeyState,
    uniform::Uniform,
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
    translation_data: Rc<RefCell<[f32; 3]>>,
    translation: Uniform<'a>,
    base_color_data: Rc<RefCell<Color>>,
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
        let factory = AttributeFactory::new(context);
        let position_data = [[0.0_f32, 0.2, 0.0], [0.2, -0.2, 0.0], [-0.2, -0.2, 0.0]];
        let position_attribute = factory.with_array(&position_data)?;
        position_attribute.associate_variable(context, &program, "position")?;

        let translation_data = Rc::new(RefCell::new([-0.5_f32, 0.0, 0.0]));
        let translation = Uniform::new_with_data(
            context,
            Rc::clone(&translation_data),
            &program,
            "translation",
        )?;
        let base_color_data = Rc::new(RefCell::new(Color::red()));
        let base_color =
            Uniform::new_with_data(context, Rc::clone(&base_color_data), &program, "baseColor")?;

        Ok(Box::new(AnimateTriangleTime {
            program,
            position: position_attribute,
            translation_data,
            translation,
            base_color_data,
            base_color,
            frame: 0,
        }))
    }
}

impl Application for AnimateTriangleTime<'_> {
    fn update(&mut self, _key_state: &KeyState) {
        let t = self.frame as f32 / 60.0;
        self.translation_data.replace_with(|old: &mut [f32; 3]| {
            old[0] = 0.75 * t.cos();
            old[1] = 0.75 * t.sin();
            *old
        });
        self.base_color_data.replace_with(|old| {
            old[0] = (t.sin() + 1.0) / 2.0;
            old[1] = ((t + 2.1).sin() + 1.0) / 2.0;
            old[2] = ((t + 4.2).sin() + 1.0) / 2.0;
            *old
        });
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
            self.position.vertex_count.try_into().unwrap(),
        );
        context.draw_arrays(
            WebGl2RenderingContext::TRIANGLE_FAN,
            0,
            self.position.vertex_count.try_into().unwrap(),
        );
    }
}
