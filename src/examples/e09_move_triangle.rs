use anyhow::Result;
use glm::Mat4;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::core::{
    application::Application,
    attribute::Attribute,
    color::Color,
    gl,
    input::KeyState,
    matrix::{self, Angle, Perspective},
    uniform::{Uniform, UniformData},
};

const VERTEX_SHADER_SOURCE: &str = r##"#version 300 es
in vec3 position;
uniform mat4 projectionMatrix;
uniform mat4 modelMatrix;
void main()
{
    gl_Position = projectionMatrix * modelMatrix * vec4(position, 1.0);
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

pub struct MoveTriangle {
    program: WebGlProgram,
    position: Attribute,
    model_matrix: Uniform,
    projection_matrix: Uniform,
    move_speed: f32,
    turn_speed: Angle,
}

impl MoveTriangle {
    const DELTA_TIME_SEC: f32 = 1_f32 / 60.0;

    pub fn create(context: &WebGl2RenderingContext) -> Result<Box<dyn Application>> {
        log!("Initializing...");
        gl::set_clear_color(context, &Color::black());
        context.enable(WebGl2RenderingContext::DEPTH_TEST);
        let program = gl::build_program(context, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE)?;
        let vao = gl::create_vertex_array(context)?;
        context.bind_vertex_array(Some(&vao));
        let position_data = [[0.0_f32, 0.2, 0.0], [0.1, -0.2, 0.0], [-0.1, -0.2, 0.0]];
        let position_attribute = Attribute::with_array(context, &position_data)?;
        position_attribute.associate_variable(context, &program, "position")?;

        let model_matrix = Uniform::new_with_data(
            context,
            UniformData::from(matrix::translation(0.0, 0.0, -1.0)),
            &program,
            "modelMatrix",
        )?;

        let projection_matrix: Uniform = Uniform::new_with_data(
            context,
            UniformData::from(Mat4::from(Perspective::default())),
            &program,
            "projectionMatrix",
        )?;

        Ok(Box::new(MoveTriangle {
            program,
            position: position_attribute,
            model_matrix,
            projection_matrix,
            move_speed: 0.5,
            turn_speed: Angle::from_degrees(90.0),
        }))
    }
}

impl Application for MoveTriangle {
    fn update(&mut self, key_state: &KeyState) {
        let move_amount = self.move_speed * Self::DELTA_TIME_SEC;
        let turn_mount = self.turn_speed * Self::DELTA_TIME_SEC;
        let model_matrix = self.model_matrix.mat4_mut().unwrap();
        // global
        if key_state.is_pressed("KeyW") {
            let m = matrix::translation(0.0, move_amount, 0.0);
            *model_matrix = m * *model_matrix;
        }
        if key_state.is_pressed("KeyS") {
            let m = matrix::translation(0.0, -move_amount, 0.0);
            *model_matrix = m * *model_matrix;
        }
        if key_state.is_pressed("KeyA") {
            let m = matrix::translation(-move_amount, 0.0, 0.0);
            *model_matrix = m * *model_matrix;
        }
        if key_state.is_pressed("KeyD") {
            let m = matrix::translation(move_amount, 0.0, 0.0);
            *model_matrix = m * *model_matrix;
        }
        if key_state.is_pressed("KeyZ") {
            let m = matrix::translation(0.0, 0.0, move_amount);
            *model_matrix = m * *model_matrix;
        }
        if key_state.is_pressed("KeyX") {
            let m = matrix::translation(0.0, 0.0, -move_amount);
            *model_matrix = m * *model_matrix;
        }
        if key_state.is_pressed("KeyQ") {
            let m = matrix::rotation_z(turn_mount);
            *model_matrix = m * *model_matrix;
        }
        if key_state.is_pressed("KeyE") {
            let m = matrix::rotation_z(-turn_mount);
            *model_matrix = m * *model_matrix;
        }
        // local
        if key_state.is_pressed("KeyI") {
            let m = matrix::translation(0.0, move_amount, 0.0);
            *model_matrix *= m;
        }
        if key_state.is_pressed("KeyK") {
            let m = matrix::translation(0.0, -move_amount, 0.0);
            *model_matrix *= m;
        }
        if key_state.is_pressed("KeyJ") {
            let m = matrix::translation(-move_amount, 0.0, 0.0);
            *model_matrix *= m;
        }
        if key_state.is_pressed("KeyL") {
            let m = matrix::translation(move_amount, 0.0, 0.0);
            *model_matrix *= m;
        }
        if key_state.is_pressed("KeyU") {
            let m = matrix::rotation_z(turn_mount);
            *model_matrix *= m;
        }
        if key_state.is_pressed("KeyO") {
            let m = matrix::rotation_z(-turn_mount);
            *model_matrix *= m;
        }
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        context.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );
        context.use_program(Some(&self.program));
        self.projection_matrix.upload_data(context);
        self.model_matrix.upload_data(context);
        context.draw_arrays(
            WebGl2RenderingContext::TRIANGLES,
            0,
            self.position.vertex_count.try_into().unwrap(),
        );
    }
}
