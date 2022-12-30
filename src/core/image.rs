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

    pub fn new_with_type(
        image_type: ImageType,
        name: Option<String>,
        mime_type: Option<String>,
    ) -> Self {
        Self {
            image_type,
            name,
            mime_type,
        }
    }

    pub async fn fetch(uri: &str) -> Result<Self> {
        let html_image = web::fetch_image(uri).await?;
        Ok(Self::new(html_image, None, None))
    }

    pub fn tex_image_2d(&self, context: &WebGl2RenderingContext) -> Result<()> {
        self.image_type.tex_image_2d(context)
    }
}

#[derive(Debug, Clone)]
enum ImageType {
    HtmlImageElement(HtmlImageElement),
    HtmlCanvasElement(HtmlCanvasElement),
}

impl ImageType {
    pub fn tex_image_2d(&self, context: &WebGl2RenderingContext) -> Result<()> {
        let internal_format = WebGl2RenderingContext::RGBA as i32;
        let format = WebGl2RenderingContext::RGBA;
        let image_type = WebGl2RenderingContext::UNSIGNED_BYTE;
        match self {
            Self::HtmlImageElement(html_image) => context
                .tex_image_2d_with_u32_and_u32_and_html_image_element(
                    WebGl2RenderingContext::TEXTURE_2D,
                    0,
                    internal_format,
                    format,
                    image_type,
                    &html_image,
                ),
            Self::HtmlCanvasElement(canvas) => context
                .tex_image_2d_with_u32_and_u32_and_html_canvas_element(
                    WebGl2RenderingContext::TEXTURE_2D,
                    0,
                    internal_format,
                    format,
                    image_type,
                    canvas,
                ),
        }
        .map_err(|error| anyhow!("Error while specifying: {:#?}", error))
    }
}

impl From<HtmlCanvasElement> for Image {
    fn from(canvas: HtmlCanvasElement) -> Self {
        Self::new_with_type(ImageType::HtmlCanvasElement(canvas), None, None)
    }
}
