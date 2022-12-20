use anyhow::Result;
use web_sys::WebGl2RenderingContext;

#[derive(Debug, Clone)]
pub struct GlDiagnostics {
    pub vendor: String,
    pub renderer: String,
    pub version: String,
    pub shading_language_version: String,
    pub max_combined_texture_image_units: i32,
    pub max_texture_image_units: i32,
    pub max_texture_size: i32,
    pub max_varying_vectors: i32,
    pub max_vertex_attribs: i32,
    pub max_vertex_texture_image_units: i32,
    pub extensions: Vec<String>,
}

impl GlDiagnostics {
    pub fn collect(context: &WebGl2RenderingContext) -> Result<Self> {
        let diagnostics = GlDiagnostics {
            vendor: super::get_string_parameter(context, WebGl2RenderingContext::VENDOR)?,
            renderer: super::get_string_parameter(context, WebGl2RenderingContext::RENDERER)?,
            version: super::get_string_parameter(context, WebGl2RenderingContext::VERSION)?,
            shading_language_version: super::get_string_parameter(
                context,
                WebGl2RenderingContext::SHADING_LANGUAGE_VERSION,
            )?,
            max_combined_texture_image_units: super::get_f64_parameter(
                context,
                WebGl2RenderingContext::MAX_COMBINED_TEXTURE_IMAGE_UNITS,
            )? as i32,
            max_texture_image_units: super::get_f64_parameter(
                context,
                WebGl2RenderingContext::MAX_TEXTURE_IMAGE_UNITS,
            )? as i32,
            max_texture_size: super::get_f64_parameter(
                context,
                WebGl2RenderingContext::MAX_TEXTURE_SIZE,
            )? as i32,
            max_varying_vectors: super::get_f64_parameter(
                context,
                WebGl2RenderingContext::MAX_VARYING_VECTORS,
            )? as i32,
            max_vertex_attribs: super::get_f64_parameter(
                context,
                WebGl2RenderingContext::MAX_VERTEX_ATTRIBS,
            )? as i32,
            max_vertex_texture_image_units: super::get_f64_parameter(
                context,
                WebGl2RenderingContext::MAX_VERTEX_TEXTURE_IMAGE_UNITS,
            )? as i32,
            extensions: super::get_supported_extensions(context),
        };
        Ok(diagnostics)
    }
}
