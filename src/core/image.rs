use anyhow::{anyhow, Result};
use web_sys::{HtmlImageElement, WebGl2RenderingContext};

#[derive(Debug, Clone)]
pub struct Image {
    html_image: HtmlImageElement,
    #[allow(dead_code)]
    name: Option<String>,
    mime_type: Option<String>,
}

impl Image {
    pub fn new(
        html_image: HtmlImageElement,
        name: Option<String>,
        mime_type: Option<String>,
    ) -> Self {
        Self {
            html_image,
            name,
            mime_type,
        }
    }

    pub fn tex_image_2d(&self, context: &WebGl2RenderingContext) -> Result<()> {
        context
            .tex_image_2d_with_u32_and_u32_and_html_image_element(
                WebGl2RenderingContext::TEXTURE_2D,
                0,
                WebGl2RenderingContext::RGBA as i32,
                WebGl2RenderingContext::RGBA,
                WebGl2RenderingContext::UNSIGNED_BYTE,
                &self.html_image,
            )
            .map_err(|error| anyhow!("Error while specifying: {:#?}", error))
    }
}
