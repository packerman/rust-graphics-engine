use anyhow::{anyhow, Result};
use web_sys::{HtmlCanvasElement, HtmlImageElement, WebGl2RenderingContext};

use crate::base::web;

#[derive(Debug, Clone)]
pub struct Image {
    image_type: ImageType,
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
            image_type: ImageType::HtmlImageElement(html_image),
            name,
            mime_type,
        }
    }

    pub async fn fetch(uri: &str) -> Result<Self> {
        let html_image = web::fetch_image(uri).await?;
        Ok(Self::new(html_image, None, None))
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

enum ImageType {
    HtmlImageElement(HtmlImageElement),
    HtmlCanvasElement(HtmlCanvasElement),
}

impl From<HtmlCanvasElement> for Image {}
