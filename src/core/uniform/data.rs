use std::{collections::HashMap, rc::Rc};

use glm::{Mat4, Vec2, Vec3, Vec4};
use web_sys::{WebGl2RenderingContext, WebGlUniformLocation};

use crate::core::{
    color::Color,
    math::resolution::Resolution,
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

    pub fn resolution(&self) -> Resolution {
        self.texture.resolution()
    }
}

#[derive(Debug, Clone)]
pub enum Basic {
    Boolean(bool),
    Int(i32),
    Float(f32),
    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4),
    Mat4(Mat4),
    Sampler2D(Sampler2D),
}

#[derive(Debug, Clone)]
pub enum Data {
    Basic { value: Basic },
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
        Self::Basic {
            value: Basic::Boolean(data),
        }
    }
}

impl From<i32> for Data {
    fn from(data: i32) -> Self {
        Self::Basic {
            value: Basic::Int(data),
        }
    }
}

impl From<f32> for Data {
    fn from(data: f32) -> Self {
        Self::Basic {
            value: Basic::Float(data),
        }
    }
}

impl From<Vec2> for Data {
    fn from(data: Vec2) -> Self {
        Self::Basic {
            value: Basic::Vec2(data),
        }
    }
}

impl From<Vec3> for Data {
    fn from(data: Vec3) -> Self {
        Self::Basic {
            value: Basic::Vec3(data),
        }
    }
}

impl From<Vec4> for Data {
    fn from(data: Vec4) -> Self {
        Self::Basic {
            value: Basic::Vec4(data),
        }
    }
}

impl From<Mat4> for Data {
    fn from(data: Mat4) -> Self {
        Self::Basic {
            value: Basic::Mat4(data),
        }
    }
}

impl From<Sampler2D> for Data {
    fn from(data: Sampler2D) -> Self {
        Self::Basic {
            value: Basic::Sampler2D(data),
        }
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

pub trait CreateDataFromValue {
    fn create_data(&self) -> Data;
}

pub trait CreateDataFromType {
    fn create_data() -> Data;
}
