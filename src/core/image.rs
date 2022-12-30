use anyhow::{anyhow, Result};
use web_sys::{HtmlCanvasElement, HtmlImageElement, WebGl2RenderingContext};

use crate::base::{math::resolution::Resolution, web};

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

    pub fn resolution(&self) -> Resolution {
        self.image_type.resolution()
    }
}

#[derive(Debug, Clone)]
enum ImageType {
    HtmlImageElement(HtmlImageElement),
    HtmlCanvasElement(HtmlCanvasElement),
    Buffer(Resolution),
}

impl ImageType {
    pub fn tex_image_2d(&self, context: &WebGl2RenderingContext) -> Result<()> {
        let target = WebGl2RenderingContext::TEXTURE_2D;
        let internal_format = WebGl2RenderingContext::RGBA as i32;
        let format = WebGl2RenderingContext::RGBA;
        let image_type = WebGl2RenderingContext::UNSIGNED_BYTE;
        match self {
            Self::HtmlImageElement(html_image) => context
                .tex_image_2d_with_u32_and_u32_and_html_image_element(
                    target,
                    0,
                    internal_format,
                    format,
                    image_type,
                    &html_image,
                ),
            Self::HtmlCanvasElement(canvas) => context
                .tex_image_2d_with_u32_and_u32_and_html_canvas_element(
                    target,
                    0,
                    internal_format,
                    format,
                    image_type,
                    canvas,
                ),
                Self::Buffer(resolution) =>
                context
                .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                    target,
                    0,
                    internal_format,
                    resolution.width,
                    resolution.height,
                    0,
                    format,
                    image_type,
                    None
                ),

        }
        .map_err(|error| anyhow!("Error while specifying: {:#?}", error))
    }

    pub fn resolution(&self) -> Resolution {
        match self {
            Self::HtmlImageElement(html_image) => {
                Resolution::new(html_image.width() as i32, html_image.height() as i32)
            }
            Self::HtmlCanvasElement(canvas) => {
                Resolution::new(canvas.width() as i32, canvas.height() as i32)
            }
            Self::Buffer(resolution) => *resolution,
        }
    }
}

impl From<HtmlCanvasElement> for Image {
    fn from(canvas: HtmlCanvasElement) -> Self {
        Self::new_with_type(ImageType::HtmlCanvasElement(canvas), None, None)
    }
}

impl From<Resolution> for Image {
    fn from(resolution: Resolution) -> Self {
        Self::new_with_type(ImageType::Buffer(resolution), None, None)
    }
}
