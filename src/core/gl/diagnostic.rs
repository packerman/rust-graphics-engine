use anyhow::Result;
use web_sys::WebGl2RenderingContext;

#[derive(Debug, Clone)]
pub struct GlDiagnostics {
    pub vendor: String,
    pub renderer: String,
    pub version: String,
    pub shading_language_version: String,
    pub max_combined_texture_image_units: f64,
    pub max_texture_size: f64,
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
            )?,
            max_texture_size: super::get_f64_parameter(
                context,
                WebGl2RenderingContext::MAX_TEXTURE_SIZE,
            )?,
            extensions: super::get_supported_extensions(context),
        };
        Ok(diagnostics)
    }
}
