use std::rc::Rc;

use anyhow::Result;
use web_sys::{WebGl2RenderingContext, WebGlFramebuffer};

use crate::{
    base::{gl, math::resolution::Resolution},
    core::texture::Texture,
};

#[derive(Debug, Clone)]
pub struct RenderTarget {
    resolution: Resolution,
    framebuffer: WebGlFramebuffer,
    texture: Rc<Texture>,
}

impl RenderTarget {
    pub fn initialize(context: &WebGl2RenderingContext, resolution: Resolution) -> Result<Self> {
        Self::initialize_with_texture(
            context,
            resolution,
            Self::create_texture(context, resolution)?,
        )
    }

    pub fn initialize_with_texture(
        context: &WebGl2RenderingContext,
        resolution: Resolution,
        texture: Rc<Texture>,
    ) -> Result<Self> {
        texture.bind(context);
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
            WebGl2RenderingContext::DEPTH_COMPONENT16,
            resolution.width,
            resolution.height,
        );
        context.framebuffer_renderbuffer(
            WebGl2RenderingContext::FRAMEBUFFER,
            WebGl2RenderingContext::DEPTH_ATTACHMENT,
            WebGl2RenderingContext::RENDERBUFFER,
            Some(&depth_buffer),
        );
        gl::check_framebuffer_status(context, WebGl2RenderingContext::FRAMEBUFFER)?;
        Ok(RenderTarget {
            resolution,
            framebuffer,
            texture,
        })
    }

    pub fn bind(&self, context: &WebGl2RenderingContext) {
        context.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, Some(&self.framebuffer));
    }

    pub fn resolution(&self) -> Resolution {
        self.resolution
    }

    pub fn texture(&self) -> Rc<Texture> {
        Rc::clone(&self.texture)
    }

    fn create_texture(
        context: &WebGl2RenderingContext,
        resolution: Resolution,
    ) -> Result<Rc<Texture>> {
        Texture::initialize(
            context,
            TextureData::new_buffer(resolution),
            TextureProperties {
                mag_filter: WebGl2RenderingContext::LINEAR as i32,
                min_filter: WebGl2RenderingContext::LINEAR as i32,
                wrap: WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
            },
        )
    }
}
