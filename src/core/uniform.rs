use std::{
    cell::{RefCell, RefMut},
    ops::Deref,
    rc::Rc,
};

use anyhow::Result;
use glm::{Mat4, Vec2, Vec3};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlUniformLocation};

use super::{
    color::Color,
    gl,
    texture::{Texture, TextureUnit},
};

#[derive(Debug, Clone)]
pub enum UniformData {
    Boolean(bool),
    Float(f32),
    Vec3(Vec3),
    Color(Color),
    Mat4(Mat4),
    Sampler2D {
        texture: Rc<Texture>,
        unit: TextureUnit,
    },
    Vec2(Vec2),
}

impl UniformData {
    pub fn sampler2d(texture: Rc<Texture>, unit: TextureUnit) -> Self {
        UniformData::Sampler2D { texture, unit }
    }

    pub fn float_mut(&mut self) -> Option<&mut f32> {
        match self {
            UniformData::Float(data) => Some(data),
            _ => None,
        }
    }

    pub fn vec3_mut(&mut self) -> Option<&mut Vec3> {
        match self {
            UniformData::Vec3(data) => Some(data),
            _ => None,
        }
    }

    pub fn color_mut(&mut self) -> Option<&mut Color> {
        match self {
            UniformData::Color(data) => Some(data),
            _ => None,
        }
    }

    pub fn mat4_mut(&mut self) -> Option<&mut Mat4> {
        match self {
            UniformData::Mat4(data) => Some(data),
            _ => None,
        }
    }
}

impl From<bool> for UniformData {
    fn from(data: bool) -> Self {
        UniformData::Boolean(data)
    }
}

impl From<f32> for UniformData {
    fn from(data: f32) -> Self {
        UniformData::Float(data)
    }
}

impl From<Vec3> for UniformData {
    fn from(data: Vec3) -> Self {
        UniformData::Vec3(data)
    }
}

impl From<Color> for UniformData {
    fn from(data: Color) -> Self {
        UniformData::Color(data)
    }
}

impl From<Mat4> for UniformData {
    fn from(data: Mat4) -> Self {
        UniformData::Mat4(data)
    }
}

impl From<Vec2> for UniformData {
    fn from(data: Vec2) -> Self {
        UniformData::Vec2(data)
    }
}

#[derive(Clone)]
pub struct Uniform {
    data: RefCell<UniformData>,
    location: WebGlUniformLocation,
}

impl Uniform {
    pub fn new_with_data(
        context: &WebGl2RenderingContext,
        data: UniformData,
        program: &WebGlProgram,
        name: &str,
    ) -> Result<Self> {
        let location = gl::get_uniform_location(context, program, name)?;
        let uniform = Uniform {
            data: RefCell::new(data),
            location,
        };
        Ok(uniform)
    }

    pub fn upload_data(&self, context: &WebGl2RenderingContext) {
        let location = Some(&self.location);
        match self.data.borrow().deref() {
            UniformData::Boolean(data) => context.uniform1i(location, i32::from(*data)),
            UniformData::Float(data) => context.uniform1f(location, *data),
            UniformData::Vec3(data) => context.uniform3f(location, data.x, data.y, data.z),
            UniformData::Color(data) => {
                context.uniform4f(location, data[0], data[1], data[2], data[3])
            }
            UniformData::Mat4(data) => {
                context.uniform_matrix4fv_with_f32_array(location, false, data.into())
            }
            UniformData::Sampler2D { texture, unit } => {
                unit.upload_data(context, location, texture.texture())
            }
            UniformData::Vec2(data) => context.uniform2f(location, data.x, data.y),
        }
    }

    pub fn float_mut(&self) -> Result<RefMut<f32>, RefMut<UniformData>> {
        RefMut::filter_map(self.data.borrow_mut(), |data| data.float_mut())
    }

    pub fn vec3_mut(&self) -> Result<RefMut<Vec3>, RefMut<UniformData>> {
        RefMut::filter_map(self.data.borrow_mut(), |data| data.vec3_mut())
    }

    pub fn color_mut(&self) -> Result<RefMut<Color>, RefMut<UniformData>> {
        RefMut::filter_map(self.data.borrow_mut(), |data| data.color_mut())
    }

    pub fn mat4_mut(&self) -> Result<RefMut<Mat4>, RefMut<UniformData>> {
        RefMut::filter_map(self.data.borrow_mut(), |data| data.mat4_mut())
    }
}
