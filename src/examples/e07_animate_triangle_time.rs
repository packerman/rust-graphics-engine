use anyhow::Result;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::core::application::Application;
use crate::core::attribute::{Attribute, DataType};
use crate::core::color::{gray, red, Color};
use crate::core::gl::{build_program, create_vertex_array, set_clear_color};
use crate::core::input::KeyState;
use crate::core::uniform::{Uniform, UploadData};

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

pub struct AnimateTriangleTime {
    program: WebGlProgram,
    vertex_count: usize,
    translation: Uniform<[f32; 3]>,
    base_color: Uniform<Color>,
    frame: usize,
}

impl AnimateTriangleTime {
    #[allow(dead_code)]
    pub fn create(context: &WebGl2RenderingContext) -> Result<Box<dyn Application>> {
        log!("Initializing...");
        set_clear_color(context, &gray());
        let program = build_program(context, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE)?;
        let vao = create_vertex_array(context)?;
        context.bind_vertex_array(Some(&vao));
        let position_data = [[0.0_f32, 0.2, 0.0], [0.2, -0.2, 0.0], [-0.2, -0.2, 0.0]];
        let position_attribute =
            Attribute::new_with_data(context, &DataType::VEC3, &position_data)?;
        position_attribute.associate_variable(context, &program, "position")?;

        let translation =
            Uniform::new_with_data(context, [-0.5_f32, 0.0, 0.0], &program, "translation")?;
        let base_color = Uniform::new_with_data(context, red(), &program, "baseColor")?;

        Ok(Box::new(AnimateTriangleTime {
            program,
            vertex_count: position_data.len(),
            translation,
            base_color,
            frame: 0,
        }))
    }
}

impl Application for AnimateTriangleTime {
    fn update(&mut self, _key_state: &KeyState) {
        let t = self.frame as f32 / 60.0;
        self.translation.data[0] = 0.75 * t.cos();
        self.translation.data[1] = 0.75 * t.sin();
        self.base_color.data[0] = (t.sin() + 1.0) / 2.0;
        self.base_color.data[1] = ((t + 2.1).sin() + 1.0) / 2.0;
        self.base_color.data[2] = ((t + 4.2).sin() + 1.0) / 2.0;
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
            self.vertex_count.try_into().unwrap(),
        );
        context.draw_arrays(
            WebGl2RenderingContext::TRIANGLE_FAN,
            0,
            self.vertex_count.try_into().unwrap(),
        );
    }
}
