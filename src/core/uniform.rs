pub mod data;

use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    ops::Deref,
};

use anyhow::{anyhow, Result};
use glm::{Mat4, Vec2, Vec3};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlUniformLocation};

use self::data::{Data, Sampler2D};

use super::color::Color;

#[derive(Debug, Clone)]
enum UniformCall {
    Boolean(bool),
    Int(i32),
    Float(f32),
    Vec3(Vec3),
    Color(Color),
    Mat4(Mat4),
    Sampler2D(Sampler2D),
    Vec2(Vec2),
}

impl UniformCall {
    pub fn as_mut_int(&mut self) -> Option<&mut i32> {
        match self {
            Self::Int(data) => Some(data),
            _ => None,
        }
    }

    pub fn as_mut_float(&mut self) -> Option<&mut f32> {
        match self {
            Self::Float(data) => Some(data),
            _ => None,
        }
    }

    pub fn as_mut_vec3(&mut self) -> Option<&mut Vec3> {
        match self {
            Self::Vec3(data) => Some(data),
            _ => None,
        }
    }

    pub fn as_mut_color(&mut self) -> Option<&mut Color> {
        match self {
            Self::Color(data) => Some(data),
            _ => None,
        }
    }

    pub fn as_mut_mat4(&mut self) -> Option<&mut Mat4> {
        match self {
            Self::Mat4(data) => Some(data),
            _ => None,
        }
    }
}

impl From<bool> for UniformCall {
    fn from(data: bool) -> Self {
        Self::Boolean(data)
    }
}

impl From<i32> for UniformCall {
    fn from(data: i32) -> Self {
        Self::Int(data)
    }
}

impl From<f32> for UniformCall {
    fn from(data: f32) -> Self {
        Self::Float(data)
    }
}

impl From<Vec2> for UniformCall {
    fn from(data: Vec2) -> Self {
        Self::Vec2(data)
    }
}

impl From<Vec3> for UniformCall {
    fn from(data: Vec3) -> Self {
        Self::Vec3(data)
    }
}

impl From<Mat4> for UniformCall {
    fn from(data: Mat4) -> Self {
        Self::Mat4(data)
    }
}

impl From<Color> for UniformCall {
    fn from(data: Color) -> Self {
        Self::Color(data)
    }
}

impl From<Sampler2D> for UniformCall {
    fn from(data: Sampler2D) -> Self {
        Self::Sampler2D(data)
    }
}

#[derive(Debug, Clone)]
pub enum Uniform {
    Basic(BasicUniform),
    Struct(StructUniform),
}

impl Uniform {
    fn initialize(
        context: &WebGl2RenderingContext,
        data: UniformCall,
        program: &WebGlProgram,
        name: &str,
    ) -> Option<Self> {
        BasicUniform::initialize(context, data, program, name).map(Self::Basic)
    }

    pub fn from_data(
        context: &WebGl2RenderingContext,
        program: &WebGlProgram,
        name: &str,
        data: Data,
    ) -> Option<Self> {
        match data {
            Data::Boolean(value) => {
                Self::initialize(context, UniformCall::from(value), program, name)
            }
            Data::Int(value) => Self::initialize(context, UniformCall::from(value), program, name),
            Data::Float(value) => {
                Self::initialize(context, UniformCall::from(value), program, name)
            }
            Data::Vec2(value) => Self::initialize(context, UniformCall::from(value), program, name),
            Data::Vec3(value) => Self::initialize(context, UniformCall::from(value), program, name),
            Data::Mat4(value) => Self::initialize(context, UniformCall::from(value), program, name),
            Data::Color(value) => {
                Self::initialize(context, UniformCall::from(value), program, name)
            }
            Data::Sampler2D(value) => {
                Self::initialize(context, UniformCall::from(value), program, name)
            }
            Data::Struct { members } => Self::from_members(context, program, name, members),
        }
    }

    pub fn try_from_data(
        context: &WebGl2RenderingContext,
        program: &WebGlProgram,
        name: &str,
        data: Data,
    ) -> Result<Self> {
        Self::from_data(context, program, name, data)
            .ok_or_else(|| anyhow!("Cannot find uniform {:#?}", name))
    }

    pub fn from_default<T>(
        context: &WebGl2RenderingContext,
        program: &WebGlProgram,
        name: &str,
    ) -> Option<Self>
    where
        T: Into<Data> + Default,
    {
        Self::from_data(context, program, name, T::default().into())
    }

    pub fn from_members(
        context: &WebGl2RenderingContext,
        program: &WebGlProgram,
        name: &str,
        members: HashMap<String, Data>,
    ) -> Option<Self> {
        StructUniform::from_members(context, program, name, members).map(Self::Struct)
    }

    pub fn upload_data(&self, context: &WebGl2RenderingContext) {
        match self {
            Self::Basic(basic) => basic.upload_data(context),
            Self::Struct(struct_uniform) => struct_uniform.upload_data(context),
        }
    }

    pub fn as_mut_int(&self) -> Option<RefMut<i32>> {
        self.as_basic().and_then(|basic| basic.as_mut_int())
    }

    pub fn as_mut_float(&self) -> Option<RefMut<f32>> {
        self.as_basic().and_then(|basic| basic.as_mut_float())
    }

    pub fn as_mut_vec3(&self) -> Option<RefMut<Vec3>> {
        self.as_basic().and_then(|basic| basic.as_mut_vec3())
    }

    pub fn as_mut_color(&self) -> Option<RefMut<Color>> {
        self.as_basic().and_then(|basic| basic.as_mut_color())
    }

    pub fn as_mut_mat4(&self) -> Option<RefMut<Mat4>> {
        self.as_basic().and_then(|basic| basic.as_mut_mat4())
    }

    pub fn as_struct(&self) -> Option<&StructUniform> {
        match self {
            Self::Struct(struct_uniform) => Some(struct_uniform),
            _ => None,
        }
    }

    fn as_basic(&self) -> Option<&BasicUniform> {
        match self {
            Self::Basic(basic) => Some(basic),
            _ => None,
        }
    }
}

pub trait UpdateUniform {
    fn update_uniform(&self, uniform: &Uniform);
}

#[derive(Debug, Clone)]
pub struct BasicUniform {
    data: RefCell<UniformCall>,
    location: WebGlUniformLocation,
}

impl BasicUniform {
    fn initialize(
        context: &WebGl2RenderingContext,
        call: UniformCall,
        program: &WebGlProgram,
        name: &str,
    ) -> Option<Self> {
        let location = context.get_uniform_location(program, name)?;
        let uniform = Self {
            data: RefCell::new(call),
            location,
        };
        Some(uniform)
    }

    pub fn upload_data(&self, context: &WebGl2RenderingContext) {
        let location = Some(&self.location);
        match self.data.borrow().deref() {
            UniformCall::Boolean(data) => context.uniform1i(location, i32::from(*data)),
            UniformCall::Int(data) => context.uniform1i(location, *data),
            UniformCall::Float(data) => context.uniform1f(location, *data),
            UniformCall::Vec3(data) => context.uniform3f(location, data.x, data.y, data.z),
            UniformCall::Color(data) => {
                context.uniform4f(location, data[0], data[1], data[2], data[3])
            }
            UniformCall::Mat4(data) => {
                context.uniform_matrix4fv_with_f32_array(location, false, data.into())
            }
            UniformCall::Sampler2D(sampler) => sampler.upload_data(context, location),
            UniformCall::Vec2(data) => context.uniform2f(location, data.x, data.y),
        }
    }

    pub fn as_mut_int(&self) -> Option<RefMut<i32>> {
        RefMut::filter_map(self.data.borrow_mut(), |data| data.as_mut_int()).ok()
    }

    pub fn as_mut_float(&self) -> Option<RefMut<f32>> {
        RefMut::filter_map(self.data.borrow_mut(), |data| data.as_mut_float()).ok()
    }

    pub fn as_mut_vec3(&self) -> Option<RefMut<Vec3>> {
        RefMut::filter_map(self.data.borrow_mut(), |data| data.as_mut_vec3()).ok()
    }

    pub fn as_mut_color(&self) -> Option<RefMut<Color>> {
        RefMut::filter_map(self.data.borrow_mut(), |data| data.as_mut_color()).ok()
    }

    pub fn as_mut_mat4(&self) -> Option<RefMut<Mat4>> {
        RefMut::filter_map(self.data.borrow_mut(), |data| data.as_mut_mat4()).ok()
    }
}

#[derive(Debug, Clone)]
pub struct StructUniform {
    members: HashMap<String, Uniform>,
}

impl StructUniform {
    pub fn new(members: HashMap<String, Uniform>) -> Self {
        Self { members }
    }

    pub fn from_members(
        context: &WebGl2RenderingContext,
        program: &WebGlProgram,
        name: &str,
        members: HashMap<String, Data>,
    ) -> Option<Self> {
        let mut result = HashMap::new();
        for (key, data) in members.into_iter() {
            let full_name = format!("{}.{}", name, &key);
            result.insert(key, Uniform::from_data(context, program, &full_name, data)?);
        }
        Some(StructUniform::new(result))
    }

    pub fn upload_data(&self, context: &WebGl2RenderingContext) {
        self.members
            .values()
            .for_each(|uniform| uniform.upload_data(context));
    }

    pub fn set_int_member(&self, name: &str, value: i32) {
        if let Some(member) = self.members.get(name) {
            if let Some(mut data) = member.as_mut_int() {
                *data = value;
            }
        }
    }

    pub fn set_float_member(&self, name: &str, value: f32) {
        if let Some(member) = self.members.get(name) {
            if let Some(mut data) = member.as_mut_float() {
                *data = value;
            }
        }
    }

    pub fn set_vec3_member(&self, name: &str, value: Vec3) {
        if let Some(member) = self.members.get(name) {
            if let Some(mut data) = member.as_mut_vec3() {
                *data = value;
            }
        }
    }

    pub fn set_mat4_member(&self, name: &str, value: Mat4) {
        if let Some(member) = self.members.get(name) {
            if let Some(mut data) = member.as_mut_mat4() {
                *data = value;
            }
        }
    }

    pub fn set_color_member(&self, name: &str, value: Color) {
        if let Some(member) = self.members.get(name) {
            if let Some(mut data) = member.as_mut_color() {
                *data = value;
            }
        }
    }
}
