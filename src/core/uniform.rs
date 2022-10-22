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
pub enum UniformData {
    Boolean(bool),
    Float(f32),
    Vec3(Vec3),
    Color(Color),
    Mat4(Mat4),
    Sampler2D(Sampler2D),
    Vec2(Vec2),
}

impl UniformData {
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

impl From<Sampler2D> for UniformData {
    fn from(data: Sampler2D) -> Self {
        UniformData::Sampler2D(data)
    }
}

impl From<Vec2> for UniformData {
    fn from(data: Vec2) -> Self {
        UniformData::Vec2(data)
    }
}

#[derive(Debug, Clone)]
pub enum Uniform {
    Basic(BasicUniform),
    Struct(StructUniform),
}

impl Uniform {
    pub fn initialize(
        context: &WebGl2RenderingContext,
        data: UniformData,
        program: &WebGlProgram,
        name: &str,
    ) -> Result<Self> {
        BasicUniform::initialize(context, data, program, name).map(Self::Basic)
    }

    pub fn try_initialize<T>(
        context: &WebGl2RenderingContext,
        program: &WebGlProgram,
        name: &str,
    ) -> Option<Self>
    where
        T: Into<UniformData> + Default,
    {
        BasicUniform::try_initialize::<T>(context, program, name).map(Self::Basic)
    }

    pub fn upload_data(&self, context: &WebGl2RenderingContext) {
        match self {
            Self::Basic(basic) => basic.upload_data(context),
            _ => todo!(),
        }
    }

    pub fn float_mut(&self) -> Option<RefMut<f32>> {
        self.get_basic().and_then(|basic| basic.float_mut())
    }

    pub fn vec3_mut(&self) -> Option<RefMut<Vec3>> {
        self.get_basic().and_then(|basic| basic.vec3_mut())
    }

    pub fn color_mut(&self) -> Option<RefMut<Color>> {
        self.get_basic().and_then(|basic| basic.color_mut())
    }

    pub fn mat4_mut(&self) -> Option<RefMut<Mat4>> {
        self.get_basic().and_then(|basic| basic.mat4_mut())
    }

    fn get_basic(&self) -> Option<&BasicUniform> {
        match self {
            Self::Basic(basic) => Some(basic),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BasicUniform {
    data: RefCell<UniformData>,
    location: WebGlUniformLocation,
}

impl BasicUniform {
    pub fn initialize(
        context: &WebGl2RenderingContext,
        data: UniformData,
        program: &WebGlProgram,
        name: &str,
    ) -> Result<Self> {
        let location = gl::get_uniform_location(context, program, name)?;
        let uniform = Self {
            data: RefCell::new(data),
            location,
        };
        Ok(uniform)
    }

    pub fn try_initialize<T>(
        context: &WebGl2RenderingContext,
        program: &WebGlProgram,
        name: &str,
    ) -> Option<Self>
    where
        T: Into<UniformData> + Default,
    {
        let location = context.get_uniform_location(program, name)?;
        let uniform = Self {
            data: RefCell::new(T::default().into()),
            location,
        };
        Some(uniform)
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
            UniformData::Sampler2D(sampler) => sampler.upload_data(context, location),
            UniformData::Vec2(data) => context.uniform2f(location, data.x, data.y),
        }
    }

    pub fn float_mut(&self) -> Option<RefMut<f32>> {
        RefMut::filter_map(self.data.borrow_mut(), |data| data.float_mut()).ok()
    }

    pub fn vec3_mut(&self) -> Option<RefMut<Vec3>> {
        RefMut::filter_map(self.data.borrow_mut(), |data| data.vec3_mut()).ok()
    }

    pub fn color_mut(&self) -> Option<RefMut<Color>> {
        RefMut::filter_map(self.data.borrow_mut(), |data| data.color_mut()).ok()
    }

    pub fn mat4_mut(&self) -> Option<RefMut<Mat4>> {
        RefMut::filter_map(self.data.borrow_mut(), |data| data.mat4_mut()).ok()
    }
}

#[derive(Debug, Clone)]
pub struct StructUniform {}

impl StructUniform {}
