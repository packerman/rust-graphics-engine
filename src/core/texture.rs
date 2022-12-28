use std::rc::Rc;

use anyhow::Result;
use web_sys::{WebGl2RenderingContext, WebGlTexture};

use crate::{base::gl, gltf::program::UpdateUniformValue};

use super::{image::Image, sampler::Sampler};

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
