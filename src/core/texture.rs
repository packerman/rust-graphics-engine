use std::rc::Rc;

use anyhow::{anyhow, Result};

use web_sys::{
    HtmlCanvasElement, HtmlImageElement, WebGl2RenderingContext, WebGlTexture, WebGlUniformLocation,
};

use crate::base::{gl, math::resolution::Resolution, web};

#[derive(Debug, Clone)]
pub struct TextureProperties {
    pub mag_filter: i32,
    pub min_filter: i32,
    pub wrap: i32,
}

impl Default for TextureProperties {
    fn default() -> Self {
        TextureProperties {
            mag_filter: WebGl2RenderingContext::LINEAR as i32,
            min_filter: WebGl2RenderingContext::LINEAR_MIPMAP_LINEAR as i32,
            wrap: WebGl2RenderingContext::REPEAT as i32,
        }
    }
}

impl TextureProperties {
    fn upload_data(&self, context: &WebGl2RenderingContext) {
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            self.mag_filter,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            self.min_filter,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_S,
            self.wrap,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_T,
            self.wrap,
        );
    }

    fn has_mipmap_filter(&self) -> bool {
        self.min_filter != WebGl2RenderingContext::LINEAR as i32
            && self.min_filter != WebGl2RenderingContext::NEAREST as i32
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TextureUnit {
    reference: u32,
    number: i32,
}

impl TextureUnit {
    pub fn upload_data(
        &self,
        context: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        texture: &WebGlTexture,
    ) {
        context.active_texture(self.reference);
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(texture));
        context.uniform1i(location, self.number);
    }
}

impl From<i32> for TextureUnit {
    fn from(i: i32) -> Self {
        TextureUnit {
            reference: WebGl2RenderingContext::TEXTURE0 + i as u32,
            number: i,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TextureData {
    Image(HtmlImageElement),
    Canvas(HtmlCanvasElement),
    Buffer(Resolution),
}

impl From<HtmlImageElement> for TextureData {
    fn from(image: HtmlImageElement) -> Self {
        TextureData::Image(image)
    }
}

impl From<HtmlCanvasElement> for TextureData {
    fn from(canvas: HtmlCanvasElement) -> Self {
        TextureData::Canvas(canvas)
    }
}

impl TextureData {
    const TYPE: u32 = WebGl2RenderingContext::UNSIGNED_BYTE;

    pub async fn load_from_source(source: &str) -> Result<Self> {
        web::fetch_image(source).await.map(Self::from)
    }

    pub fn new_buffer(resolution: Resolution) -> Self {
        Self::Buffer(resolution)
    }

    pub fn tex_image_2d(
        &self,
        context: &WebGl2RenderingContext,
        target: u32,
        internal_format: i32,
        format: u32,
    ) -> Result<()> {
        let result = match self {
            Self::Image(image) => context
                .tex_image_2d_with_u32_and_u32_and_html_image_element(
                    target,
                    0,
                    internal_format,
                    format,
                    Self::TYPE,
                    image,
                ),
                Self::Canvas(canvas) => context
                .tex_image_2d_with_u32_and_u32_and_html_canvas_element(
                    target,
                    0,
                    internal_format,
                    format,
                    Self::TYPE,
                    canvas,
                ),
                Self::Buffer(resolution) => context
                .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                    target,
                    0,
                    internal_format,
                    resolution.width,
                    resolution.height,
                    0,
                    format,
                    Self::TYPE,
                    None
                ),
        };
        result.map_err(|err| anyhow!("Error when uploading texture data: {:#?}", err))
    }

    pub fn resolution(&self) -> Resolution {
        match self {
            Self::Buffer(resolution) => *resolution,
            Self::Canvas(canvas) => Resolution::new(canvas.width() as i32, canvas.height() as i32),
            Self::Image(image) => Resolution::new(image.width() as i32, image.height() as i32),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Texture {
    texture: WebGlTexture,
    properties: TextureProperties,
    data: TextureData,
}

impl Texture {
    pub fn initialize(
        context: &WebGl2RenderingContext,
        data: TextureData,
        properties: TextureProperties,
    ) -> Result<Rc<Self>> {
        let texture = gl::create_texture(context)?;
        let texture = Texture {
            texture,
            data,
            properties,
        };
        texture.upload_data(context)?;
        Ok(Rc::new(texture))
    }

    pub fn bind(&self, context: &WebGl2RenderingContext) {
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(self.texture()));
    }

    pub fn texture(&self) -> &WebGlTexture {
        &self.texture
    }

    pub fn upload_data(&self, context: &WebGl2RenderingContext) -> Result<()> {
        self.bind(context);
        self.data.tex_image_2d(
            context,
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::RGBA as i32,
            WebGl2RenderingContext::RGBA,
        )?;
        if self.properties.has_mipmap_filter() {
            context.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
        }
        self.properties.upload_data(context);
        Ok(())
    }

    pub fn resolution(&self) -> Resolution {
        self.data.resolution()
    }
}
