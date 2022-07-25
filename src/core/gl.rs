use anyhow::anyhow;
use anyhow::Ok;
use anyhow::Result;

use web_sys::WebGl2RenderingContext;
use web_sys::WebGlProgram;
use web_sys::WebGlShader;
use web_sys::WebGlVertexArrayObject;

use super::color::Color;

pub fn set_clear_color(context: &WebGl2RenderingContext, color: &Color) {
    context.clear_color(color.x, color.y, color.z, color.w)
}

pub fn get_string_parameter(context: &WebGl2RenderingContext, pname: u32) -> Result<String> {
    let value = context
        .get_parameter(pname)
        .map_err(|err| anyhow!("Cannot get parameter {:#?}: {:#?}", pname, err))?;
    value
        .as_string()
        .ok_or_else(|| anyhow!("Cannot convert {:#?} to string", value))
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| anyhow!("Cannot create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);
    let compile_success = context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false);
    if compile_success {
        Ok(shader)
    } else {
        let info_log = context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader"));
        context.delete_shader(Some(&shader));
        Err(anyhow!(info_log))
    }
}

pub fn link_program(
    context: &WebGl2RenderingContext,
    vertex_shader: &WebGlShader,
    fragment_shader: &WebGlShader,
) -> Result<WebGlProgram> {
    let program = context
        .create_program()
        .ok_or_else(|| anyhow!("Cannot create program object"))?;
    context.attach_shader(&program, vertex_shader);
    context.attach_shader(&program, fragment_shader);
    context.link_program(&program);
    let link_success = context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false);
    if link_success {
        Ok(program)
    } else {
        let info_log = context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program"));
        context.delete_program(Some(&program));
        Err(anyhow!(info_log))
    }
}

pub fn build_program(
    context: &WebGl2RenderingContext,
    vertex_shader_source: &str,
    fragment_shader_source: &str,
) -> Result<WebGlProgram> {
    let vertex_shader = compile_shader(
        context,
        WebGl2RenderingContext::VERTEX_SHADER,
        vertex_shader_source,
    )?;
    let fragment_shader = compile_shader(
        context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        fragment_shader_source,
    )?;
    link_program(context, &vertex_shader, &fragment_shader)
}

pub fn create_vertex_array(context: &WebGl2RenderingContext) -> Result<WebGlVertexArrayObject> {
    context
        .create_vertex_array()
        .ok_or_else(|| anyhow!("Cannot create vertex array object"))
}
