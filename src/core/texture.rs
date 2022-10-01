use std::{rc::Rc, sync::Mutex};

use anyhow::{anyhow, Ok, Result};
use futures::channel::oneshot;
use wasm_bindgen::{prelude::*, JsCast, JsValue};
use web_sys::{
    HtmlCanvasElement, HtmlImageElement, WebGl2RenderingContext, WebGlTexture, WebGlUniformLocation,
};

use super::{gl, web};

#[derive(Debug, Clone)]
pub struct TextureProperties {
    mag_filter: i32,
    min_filter: i32,
    wrap: i32,
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
}

#[derive(Debug, Clone)]
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
        self::load_image(source).await.map(Self::from)
    }

    pub fn tex_image_2d(
        &self,
        context: &WebGl2RenderingContext,
        target: u32,
        internal_format: i32,
        format: u32,
    ) -> Result<()> {
        let result = match self {
            TextureData::Image(image) => context
                .tex_image_2d_with_u32_and_u32_and_html_image_element(
                    target,
                    0,
                    internal_format,
                    format,
                    Self::TYPE,
                    image,
                ),
            TextureData::Canvas(canvas) => context
                .tex_image_2d_with_u32_and_u32_and_html_canvas_element(
                    target,
                    0,
                    internal_format,
                    format,
                    Self::TYPE,
                    canvas,
                ),
        };
        result.map_err(|err| anyhow!("Error when uploading texture data: {:#?}", err))
    }
}

#[derive(Debug, Clone)]
pub struct Texture {
    texture: WebGlTexture,
    properties: TextureProperties,
    data: TextureData,
}

impl Texture {
    pub fn new(
        context: &WebGl2RenderingContext,
        data: TextureData,
        properties: TextureProperties,
    ) -> Result<Self> {
        let texture = gl::create_texture(context)?;
        let texture = Texture {
            texture,
            data,
            properties,
        };
        texture.upload_data(context)?;
        Ok(texture)
    }

    pub fn texture(&self) -> &WebGlTexture {
        &self.texture
    }

    pub fn upload_data(&self, context: &WebGl2RenderingContext) -> Result<()> {
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(self.texture()));
        self.data.tex_image_2d(
            context,
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::RGBA as i32,
            WebGl2RenderingContext::RGBA,
        )?;
        context.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
        self.properties.upload_data(context);
        Ok(())
    }
}

async fn load_image(source: &str) -> Result<HtmlImageElement> {
    let image = web::new_image()?;
    let (sender, receiver) = oneshot::channel::<Result<()>>();
    let success = Rc::new(Mutex::new(Some(sender)));
    let error = Rc::clone(&success);
    let success_callback = web::closure_once(move || {
        if let Some(success) = success.lock().ok().and_then(|mut success| success.take()) {
            if let Err(err) = success.send(Ok(())) {
                error!("Cannot send 'image loaded messsage': {:#?}", err);
            }
        }
    });
    let error_callback: Closure<dyn FnMut(JsValue)> = web::closure_once(move |err| {
        if let Some(error) = error.lock().ok().and_then(|mut error| error.take()) {
            if let Err(err) = error.send(Err(anyhow!("Error when loading image: {:#?}", err))) {
                error!("Cannot send 'image error message': {:#?}", err);
            }
        }
    });
    image.set_onload(Some(success_callback.as_ref().unchecked_ref()));
    image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
    image.set_src(source);

    receiver.await??;

    Ok(image)
}
