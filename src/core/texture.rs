use std::{rc::Rc, sync::Mutex};

use anyhow::{anyhow, Ok, Result};
use futures::channel::oneshot;
use wasm_bindgen::{prelude::*, JsCast, JsValue};
use web_sys::{HtmlImageElement, WebGl2RenderingContext, WebGlTexture, WebGlUniformLocation};

use super::{gl, web};

#[derive(Clone, Copy)]
pub struct Properties {
    mag_filter: i32,
    min_filter: i32,
    wrap: i32,
}

impl Default for Properties {
    fn default() -> Self {
        Properties {
            mag_filter: WebGl2RenderingContext::LINEAR as i32,
            min_filter: WebGl2RenderingContext::LINEAR_MIPMAP_LINEAR as i32,
            wrap: WebGl2RenderingContext::REPEAT as i32,
        }
    }
}

#[derive(Clone, Copy)]
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

#[derive(Clone)]
pub struct Texture {
    texture: WebGlTexture,
    properties: Properties,
    source: HtmlImageElement,
}

impl Texture {
    pub fn new_initialized(
        context: &WebGl2RenderingContext,
        source: HtmlImageElement,
        properties: Properties,
    ) -> Result<Self> {
        let texture = gl::create_texture(context)?;
        let texture = Texture {
            texture,
            source,
            properties,
        };
        texture.upload_data(context)?;
        Ok(texture)
    }

    pub async fn load_from_source(
        context: &WebGl2RenderingContext,
        source: &str,
        properties: Properties,
    ) -> Result<Self> {
        self::load_image(source)
            .await
            .and_then(|image| Self::new_initialized(context, image, properties))
    }

    pub fn texture(&self) -> &WebGlTexture {
        &self.texture
    }

    pub fn upload_data(&self, context: &WebGl2RenderingContext) -> Result<()> {
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(self.texture()));
        context
            .tex_image_2d_with_u32_and_u32_and_html_image_element(
                WebGl2RenderingContext::TEXTURE_2D,
                0,
                WebGl2RenderingContext::RGBA as i32,
                WebGl2RenderingContext::RGBA,
                WebGl2RenderingContext::UNSIGNED_BYTE,
                &self.source,
            )
            .map_err(|err| anyhow!("Error when uploading texture data: {:#?}", err))?;
        context.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            self.properties.mag_filter,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            self.properties.min_filter,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_S,
            self.properties.wrap,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_T,
            self.properties.wrap,
        );
        Ok(())
    }
}

pub async fn load_image(source: &str) -> Result<HtmlImageElement> {
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
