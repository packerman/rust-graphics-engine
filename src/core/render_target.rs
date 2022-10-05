use anyhow::Result;
use web_sys::{WebGl2RenderingContext, WebGlFramebuffer};

use super::{
    gl,
    texture::{Texture, TextureData, TextureProperties},
};

#[derive(Debug, Clone)]
pub struct RenderTarget {
    width: i32,
    height: i32,
    framebuffer: WebGlFramebuffer,
}

impl RenderTarget {
    pub fn bind(&self, context: &WebGl2RenderingContext) {
        context.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, Some(&self.framebuffer));
    }

    pub fn size(&self) -> (i32, i32) {
        (self.width, self.height)
    }
}

impl RenderTarget {
    pub fn new(context: &WebGl2RenderingContext, width: i32, height: i32) -> Result<Self> {
        Self::new_with_texture(
            context,
            width,
            height,
            Self::create_texture(context, width, height)?,
        )
    }

    pub fn new_with_texture(
        context: &WebGl2RenderingContext,
        width: i32,
        height: i32,
        texture: Texture,
    ) -> Result<Self> {
        let framebuffer = gl::create_framebuffer(context)?;
        context.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, Some(&framebuffer));
        context.framebuffer_texture_2d(
            WebGl2RenderingContext::FRAMEBUFFER,
            WebGl2RenderingContext::COLOR_ATTACHMENT0,
            WebGl2RenderingContext::TEXTURE_2D,
            Some(texture.texture()),
            0,
        );
        let depth_buffer = gl::create_renderbuffer(context)?;
        context.bind_renderbuffer(WebGl2RenderingContext::RENDERBUFFER, Some(&depth_buffer));
        context.renderbuffer_storage(
            WebGl2RenderingContext::RENDERBUFFER,
            WebGl2RenderingContext::DEPTH_COMPONENT,
            width,
            height,
        );
        context.framebuffer_renderbuffer(
            WebGl2RenderingContext::FRAMEBUFFER,
            WebGl2RenderingContext::DEPTH_ATTACHMENT,
            WebGl2RenderingContext::RENDERBUFFER,
            Some(&depth_buffer),
        );
        gl::check_framebuffer_status(context, WebGl2RenderingContext::FRAMEBUFFER)?;
        Ok(RenderTarget {
            width,
            height,
            framebuffer,
        })
    }

    fn create_texture(
        context: &WebGl2RenderingContext,
        width: i32,
        height: i32,
    ) -> Result<Texture> {
        Texture::new(
            context,
            TextureData::new_buffer(width, height),
            TextureProperties {
                mag_filter: WebGl2RenderingContext::LINEAR as i32,
                min_filter: WebGl2RenderingContext::LINEAR as i32,
                wrap: WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
            },
        )
    }
}
