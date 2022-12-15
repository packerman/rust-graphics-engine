use std::{collections::HashMap, fmt::Debug};

use anyhow::Result;
use glm::{Mat4, Vec4};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlUniformLocation};

use crate::core::{convert::FromWithContext, gl};

use super::level::Level;

#[derive(Debug, Clone)]
pub struct Uniform {
    pub location: WebGlUniformLocation,
    pub uniform_type: u32,
}

pub trait UpdateUniforms: Debug {
    fn update_uniforms(&self, context: &WebGl2RenderingContext, program: &Program);

    fn vertex_shader(&self) -> &str;

    fn fragment_shader(&self) -> &str;
}

pub trait UpdateUniform {
    fn update_uniform(&self, context: &WebGl2RenderingContext, name: &str, program: &Program) {
        self.update_uniform_with_level(context, name, program, Level::Ignore);
    }

    fn update_uniform_with_level(
        &self,
        context: &WebGl2RenderingContext,
        name: &str,
        program: &Program,
        level: Level,
    );
}

pub trait UpdateUniformValue {
    fn update_uniform_value(
        &self,
        context: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
    );

    fn value_type(&self) -> u32;
}

impl<U: UpdateUniformValue> UpdateUniform for U {
    fn update_uniform_with_level(
        &self,
        context: &WebGl2RenderingContext,
        name: &str,
        program: &Program,
        level: Level,
    ) {
        if let Some(uniform) = program.get_uniform(name) {
            level.assert(uniform.uniform_type == self.value_type(), || {
                format!(
                    "Incompatible types of uniform value '{}': uniform type = {}, value_type = {}",
                    name,
                    uniform.uniform_type,
                    self.value_type(),
                )
            });
            self.update_uniform_value(context, Some(&uniform.location));
        } else {
            level.error(|| format!("Uniform '{}' not found", name));
        }
    }
}

impl UpdateUniformValue for Vec4 {
    fn update_uniform_value(
        &self,
        context: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) {
        context.uniform4f(location, self.x, self.y, self.z, self.w)
    }

    fn value_type(&self) -> u32 {
        WebGl2RenderingContext::FLOAT_VEC4
    }
}

impl UpdateUniformValue for Mat4 {
    fn update_uniform_value(
        &self,
        context: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) {
        context.uniform_matrix4fv_with_f32_array(location, false, self.into());
    }

    fn value_type(&self) -> u32 {
        WebGl2RenderingContext::FLOAT_MAT4
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    program: WebGlProgram,
    uniforms: HashMap<String, Uniform>,
    attributes: HashMap<String, u32>,
}

impl Program {
    pub fn initialize(
        context: &WebGl2RenderingContext,
        vertex_shader: &str,
        fragment_shader: &str,
    ) -> Result<Self> {
        let program = gl::build_program(context, vertex_shader, fragment_shader)?;
        Program::from_with_context(context, program)
    }

    pub fn use_program(&self, context: &WebGl2RenderingContext) {
        context.use_program(Some(&self.program));
    }

    pub fn get_uniform(&self, name: &str) -> Option<&Uniform> {
        self.uniforms.get(name)
    }

    pub fn get_attribute_location(&self, name: &str) -> Option<&u32> {
        self.attributes.get(name)
    }

    fn get_active_uniforms(
        context: &WebGl2RenderingContext,
        program: &WebGlProgram,
    ) -> HashMap<String, Uniform> {
        let uniform_count = context
            .get_program_parameter(program, WebGl2RenderingContext::ACTIVE_UNIFORMS)
            .as_f64()
            .unwrap_or_default() as u32;
        let mut result = HashMap::new();
        for i in 0..uniform_count {
            if let Some(active_info) = context.get_active_uniform(program, i) {
                if let Some(location) = context.get_uniform_location(program, &active_info.name()) {
                    result.insert(
                        active_info.name(),
                        Uniform {
                            location,
                            uniform_type: active_info.type_(),
                        },
                    );
                }
            }
        }
        result
    }

    fn get_active_attributes(
        context: &WebGl2RenderingContext,
        program: &WebGlProgram,
    ) -> HashMap<String, u32> {
        let attribute_count = context
            .get_program_parameter(program, WebGl2RenderingContext::ACTIVE_ATTRIBUTES)
            .as_f64()
            .unwrap_or_default() as u32;
        let mut result = HashMap::new();
        for i in 0..attribute_count {
            if let Some(active_info) = context.get_active_attrib(program, i) {
                let location = context.get_attrib_location(program, &active_info.name());
                if location >= 0 {
                    result.insert(active_info.name(), location as u32);
                }
            }
        }
        result
    }
}

impl FromWithContext<WebGl2RenderingContext, WebGlProgram> for Program {
    fn from_with_context(context: &WebGl2RenderingContext, program: WebGlProgram) -> Result<Self> {
        let uniforms = Self::get_active_uniforms(context, &program);
        let attributes = Self::get_active_attributes(context, &program);
        Ok(Program {
            program,
            uniforms,
            attributes,
        })
    }
}
