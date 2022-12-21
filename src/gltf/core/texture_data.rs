use std::rc::Rc;

use anyhow::{anyhow, Result};
use web_sys::{HtmlImageElement, WebGl2RenderingContext, WebGlTexture};

use crate::{base::gl, gltf::util::validate};

#[derive(Debug, Clone)]
pub struct Sampler {
    mag_filter: Option<i32>,
    min_filter: Option<i32>,
    wrap_s: i32,
    wrap_t: i32,
}

impl Default for Sampler {
    fn default() -> Self {
        Self {
            mag_filter: None,
            min_filter: None,
            wrap_s: WebGl2RenderingContext::REPEAT as i32,
            wrap_t: WebGl2RenderingContext::REPEAT as i32,
        }
    }
}

impl Sampler {
    const MAG_FILTERS: [i32; 2] = [
        WebGl2RenderingContext::NEAREST as i32,
        WebGl2RenderingContext::LINEAR as i32,
    ];

    const MIN_FILTERS: [i32; 6] = [
        WebGl2RenderingContext::NEAREST as i32,
        WebGl2RenderingContext::LINEAR as i32,
        WebGl2RenderingContext::NEAREST_MIPMAP_NEAREST as i32,
        WebGl2RenderingContext::LINEAR_MIPMAP_NEAREST as i32,
        WebGl2RenderingContext::NEAREST_MIPMAP_LINEAR as i32,
        WebGl2RenderingContext::LINEAR_MIPMAP_LINEAR as i32,
    ];

    const WRAP: [i32; 3] = [
        WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        WebGl2RenderingContext::MIRRORED_REPEAT as i32,
        WebGl2RenderingContext::REPEAT as i32,
    ];

    pub fn new(
        mag_filter: Option<i32>,
        min_filter: Option<i32>,
        wrap_s: i32,
        wrap_t: i32,
    ) -> Result<Self> {
        validate::optional(&mag_filter, |mag_filter| {
            validate::contains(mag_filter, &Self::MAG_FILTERS, |value| {
                anyhow!("Unknown mag filter: {}", value)
            })
        })?;
        validate::optional(&min_filter, |min_filter| {
            validate::contains(min_filter, &Self::MIN_FILTERS, |value| {
                anyhow!("Unknown min filter: {}", value)
            })
        })?;
        validate::contains(&wrap_s, &Self::WRAP, |value| {
            anyhow!("Unknown wrap s parameter: {}", value)
        })?;
        validate::contains(&wrap_t, &Self::WRAP, |value| {
            anyhow!("Unknown wrap t parameter: {}", value)
        })?;
        Ok(Self {
            mag_filter,
            min_filter,
            wrap_s,
            wrap_t,
        })
    }

    pub fn set_texture_parameters(&self, context: &WebGl2RenderingContext) {
        if let Some(mag_filter) = self.mag_filter {
            context.tex_parameteri(
                WebGl2RenderingContext::TEXTURE_2D,
                WebGl2RenderingContext::TEXTURE_MAG_FILTER,
                mag_filter,
            );
        }
        if let Some(min_filter) = self.min_filter {
            context.tex_parameteri(
                WebGl2RenderingContext::TEXTURE_2D,
                WebGl2RenderingContext::TEXTURE_MIN_FILTER,
                min_filter,
            );
        }
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_S,
            self.wrap_s,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_T,
            self.wrap_t,
        );
    }

    pub fn has_mipmap_filter(&self) -> bool {
        self.min_filter.map_or(false, |min_filter| {
            let min_filter = min_filter as u32;
            min_filter == WebGl2RenderingContext::NEAREST_MIPMAP_NEAREST
                || min_filter == WebGl2RenderingContext::LINEAR_MIPMAP_NEAREST
                || min_filter == WebGl2RenderingContext::NEAREST_MIPMAP_LINEAR
                || min_filter == WebGl2RenderingContext::LINEAR_MIPMAP_LINEAR
        })
    }

    pub fn generate_mipmap(&self, context: &WebGl2RenderingContext) {
        if self.has_mipmap_filter() {
            context.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    html_image: HtmlImageElement,
}

impl Image {
    pub fn new(html_image: HtmlImageElement) -> Self {
        Self { html_image }
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
            .map_err(|error| anyhow::anyhow!("Error while specifying: {:#?}", error))
    }
}

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
