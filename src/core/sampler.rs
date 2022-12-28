use anyhow::{anyhow, Result};
use web_sys::WebGl2RenderingContext;

use crate::base::util::validate;

#[derive(Debug, Clone)]
pub struct Sampler {
    mag_filter: i32,
    min_filter: i32,
    wrap_s: i32,
    wrap_t: i32,
}

impl Default for Sampler {
    fn default() -> Self {
        Self {
            mag_filter: Self::DEFAULT_MAG_FILTER,
            min_filter: Self::DEFAULT_MIN_FILTER,
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

    const DEFAULT_MAG_FILTER: i32 = WebGl2RenderingContext::LINEAR as i32;
    const DEFAULT_MIN_FILTER: i32 = WebGl2RenderingContext::LINEAR_MIPMAP_LINEAR as i32;

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
            mag_filter: mag_filter.unwrap_or(Self::DEFAULT_MAG_FILTER),
            min_filter: min_filter.unwrap_or(Self::DEFAULT_MIN_FILTER),
            wrap_s,
            wrap_t,
        })
    }

    pub fn set_texture_parameters(&self, context: &WebGl2RenderingContext) {
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
            self.wrap_s,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_T,
            self.wrap_t,
        );
    }

    pub fn has_mipmap_filter(&self) -> bool {
        let min_filter = self.min_filter as u32;
        min_filter == WebGl2RenderingContext::NEAREST_MIPMAP_NEAREST
            || min_filter == WebGl2RenderingContext::LINEAR_MIPMAP_NEAREST
            || min_filter == WebGl2RenderingContext::NEAREST_MIPMAP_LINEAR
            || min_filter == WebGl2RenderingContext::LINEAR_MIPMAP_LINEAR
    }

    pub fn generate_mipmap(&self, context: &WebGl2RenderingContext) {
        if self.has_mipmap_filter() {
            context.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
        }
    }
}
