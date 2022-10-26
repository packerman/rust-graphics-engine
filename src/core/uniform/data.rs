use std::{collections::HashMap, rc::Rc};

use glm::{Mat4, Vec2, Vec3};
use web_sys::{WebGl2RenderingContext, WebGlUniformLocation};

use crate::core::{
    color::Color,
    texture::{Texture, TextureUnit},
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

    pub fn upload_data(
        &self,
        context: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) {
        self.unit
            .upload_data(context, location, self.texture.texture());
    }
}

pub enum Data {
    Boolean(bool),
    Float(f32),
    Vec2(Vec2),
    Vec3(Vec3),
    Mat4(Mat4),
    Color(Color),
    Sampler2D(Sampler2D),
    Struct { members: HashMap<String, Data> },
}

impl From<bool> for Data {
    fn from(data: bool) -> Self {
        Self::Boolean(data)
    }
}

impl From<f32> for Data {
    fn from(data: f32) -> Self {
        Self::Float(data)
    }
}

impl From<Vec2> for Data {
    fn from(data: Vec2) -> Self {
        Self::Vec2(data)
    }
}

impl From<Vec3> for Data {
    fn from(data: Vec3) -> Self {
        Self::Vec3(data)
    }
}

impl From<Mat4> for Data {
    fn from(data: Mat4) -> Self {
        Self::Mat4(data)
    }
}

impl From<Color> for Data {
    fn from(data: Color) -> Self {
        Self::Color(data)
    }
}

impl From<Sampler2D> for Data {
    fn from(data: Sampler2D) -> Self {
        Self::Sampler2D(data)
    }
}
