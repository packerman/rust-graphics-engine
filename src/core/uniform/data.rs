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

#[derive(Debug, Clone)]
pub enum Data {
    Boolean(bool),
    Int(i32),
    Float(f32),
    Vec2(Vec2),
    Vec3(Vec3),
    Mat4(Mat4),
    Color(Color),
    Sampler2D(Sampler2D),
    Struct { members: HashMap<String, Data> },
}

impl Data {
    pub fn default<T>() -> Self
    where
        T: Default,
        Self: From<T>,
    {
        Self::from(T::default())
    }
}

impl From<bool> for Data {
    fn from(data: bool) -> Self {
        Self::Boolean(data)
    }
}

impl From<i32> for Data {
    fn from(data: i32) -> Self {
        Self::Int(data)
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

impl<const N: usize> From<[(&str, Data); N]> for Data {
    fn from(members: [(&str, Data); N]) -> Self {
        Self::Struct {
            members: members
                .into_iter()
                .map(|(member, data)| (String::from(member), data))
                .collect(),
        }
    }
}

impl From<HashMap<&str, Data>> for Data {
    fn from(members: HashMap<&str, Data>) -> Self {
        Data::Struct {
            members: members
                .into_iter()
                .map(|(member, data)| (String::from(member), data))
                .collect(),
        }
    }
}
