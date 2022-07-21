use anyhow::anyhow;
use anyhow::Result;

use web_sys::WebGl2RenderingContext;
use web_sys::WebGlShader;

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
    let compile_successful = context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false);
    if compile_successful {
        Ok(shader)
    } else {
        let info_log = context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader"));
        context.delete_shader(Some(&shader));
        Err(anyhow!(info_log))
    }
}
