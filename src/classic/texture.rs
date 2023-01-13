use std::rc::Rc;

use web_sys::{WebGl2RenderingContext, WebGlUniformLocation};

use crate::{
    base::math::resolution::Resolution,
    core::{
        program::UpdateUniformValue,
        texture::{Texture, TextureUnit},
    },
};

#[derive(Debug, Clone)]
pub struct Sampler2D {
    pub texture: Rc<Texture>,
    unit: TextureUnit,
}

impl Sampler2D {
    pub fn new(texture: Rc<Texture>, unit: TextureUnit) -> Self {
        Self { texture, unit }
    }

    pub fn resolution(&self) -> Resolution {
        self.texture.resolution()
    }
}

impl UpdateUniformValue for Sampler2D {
    fn update_uniform_value(
        &self,
        context: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) {
        self.unit.active_texture(context);
        self.texture.bind(context);
        self.unit.update_uniform_value(context, location);
    }

    fn value_type(&self) -> u32 {
        WebGl2RenderingContext::SAMPLER_2D
    }
}
