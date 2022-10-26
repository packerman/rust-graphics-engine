use std::convert::TryInto;

use anyhow::{anyhow, Result};

use wasm_bindgen::JsValue;
use web_sys::{
    WebGl2RenderingContext, WebGlBuffer, WebGlFramebuffer, WebGlProgram, WebGlRenderbuffer,
    WebGlShader, WebGlTexture, WebGlVertexArrayObject,
};

use super::color::Color;

pub fn set_clear_color(context: &WebGl2RenderingContext, color: &Color) {
    context.clear_color(color[0], color[1], color[2], color[3]);
}

fn get_parameter(context: &WebGl2RenderingContext, pname: u32) -> Result<JsValue> {
    context
        .get_parameter(pname)
        .map_err(|err| anyhow!("Cannot get parameter {:#?}: {:#?}", pname, err))
}

pub fn get_string_parameter(context: &WebGl2RenderingContext, pname: u32) -> Result<String> {
    get_parameter(context, pname)?
        .as_string()
        .ok_or_else(|| anyhow!("Cannot convert {:#?} to string", pname))
}

pub fn get_f64_parameter(context: &WebGl2RenderingContext, pname: u32) -> Result<f64> {
    get_parameter(context, pname)?
        .as_f64()
        .ok_or_else(|| anyhow!("Cannot convert {:#?} to string", pname))
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

pub fn create_buffer(context: &WebGl2RenderingContext) -> Result<WebGlBuffer> {
    context
        .create_buffer()
        .ok_or_else(|| anyhow!("Cannot create buffer"))
}

pub fn create_vertex_array(context: &WebGl2RenderingContext) -> Result<WebGlVertexArrayObject> {
    context
        .create_vertex_array()
        .ok_or_else(|| anyhow!("Cannot create vertex array object"))
}

pub fn get_attrib_location(
    context: &WebGl2RenderingContext,
    program: &WebGlProgram,
    variable: &str,
) -> Option<u32> {
    let location = context.get_attrib_location(program, variable);
    if location == -1 {
        None
    } else {
        location.try_into().ok()
    }
}

pub fn create_texture(context: &WebGl2RenderingContext) -> Result<WebGlTexture> {
    context
        .create_texture()
        .ok_or_else(|| anyhow!("Cannot create texture"))
}

pub fn create_framebuffer(context: &WebGl2RenderingContext) -> Result<WebGlFramebuffer> {
    context
        .create_framebuffer()
        .ok_or_else(|| anyhow!("Cannot create framebuffer"))
}

pub fn create_renderbuffer(context: &WebGl2RenderingContext) -> Result<WebGlRenderbuffer> {
    context
        .create_renderbuffer()
        .ok_or_else(|| anyhow!("Cannot create renderbuffer"))
}

pub fn check_framebuffer_status(context: &WebGl2RenderingContext, target: u32) -> Result<()> {
    let status = context.check_framebuffer_status(target);
    if status == WebGl2RenderingContext::FRAMEBUFFER_COMPLETE {
        Ok(())
    } else {
        Err(anyhow!("Framebuffer error: {:#?}", status))
    }
}
