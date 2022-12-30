use std::rc::Rc;

use anyhow::Result;
use web_sys::{WebGl2RenderingContext, WebGlTexture};

use crate::base::{gl, math::resolution::Resolution};

use super::{image::Image, program::UpdateUniformValue, sampler::Sampler};

#[derive(Debug, Clone)]
pub struct Texture {
    texture: WebGlTexture,
    sampler: Rc<Sampler>,
    source: Rc<Image>,
}

impl Texture {
    pub fn initialize(
        context: &WebGl2RenderingContext,
        sampler: Rc<Sampler>,
        source: Rc<Image>,
    ) -> Result<Self> {
        let texture = gl::create_texture(context)?;
        let me = Self {
            texture,
            sampler,
            source,
        };
        me.store_data(context)?;
        Ok(me)
    }

    pub async fn fetch(context: &WebGl2RenderingContext, uri: &str) -> Result<Self> {
        let image = Rc::new(Image::fetch(uri).await?);
        Self::initialize(context, Rc::default(), image)
    }

    pub fn texture(&self) -> &WebGlTexture {
        &self.texture
    }

    pub fn bind(&self, context: &WebGl2RenderingContext) {
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&self.texture));
    }

    pub fn store_data(&self, context: &WebGl2RenderingContext) -> Result<()> {
        self.bind(context);
        self.source.tex_image_2d(context)?;
        self.sampler.set_texture_parameters(context);
        self.sampler.generate_mipmap(context);
        Ok(())
    }

    pub fn resolution(&self) -> Resolution {
        self.source.resolution()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TextureUnit(pub u32);

impl TextureUnit {
    pub fn active_texture(&self, context: &WebGl2RenderingContext) {
        context.active_texture(WebGl2RenderingContext::TEXTURE0 + self.0)
    }
}

impl UpdateUniformValue for TextureUnit {
    fn update_uniform_value(
        &self,
        context: &WebGl2RenderingContext,
        location: Option<&web_sys::WebGlUniformLocation>,
    ) {
        context.uniform1ui(location, self.0)
    }

    fn value_type(&self) -> u32 {
        WebGl2RenderingContext::SAMPLER_2D
    }
}
