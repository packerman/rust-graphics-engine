use anyhow::{anyhow, Result};
use glm::Mat4;
use js_sys::Float32Array;
use na::SVector;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram};

use super::{color::Color, gl};

#[derive(Debug, PartialEq, Eq)]
pub struct DataKind {
    size: i32,
    gl_type: u32,
}

impl DataKind {
    const fn new(size: i32, gl_type: u32) -> DataKind {
        DataKind { size, gl_type }
    }
}

pub struct AttributeData {
    data: Vec<f32>,
    kind: DataKind,
    count: i32,
}

impl AttributeData {
    pub fn new(data: Vec<f32>, kind: DataKind, count: i32) -> Self {
        Self { data, kind, count }
    }

    fn new_with_flat_array(data: Vec<f32>, size: usize, length: usize) -> Self {
        Self::new(
            data,
            DataKind::new(size.try_into().unwrap(), WebGl2RenderingContext::FLOAT),
            length.try_into().unwrap(),
        )
    }

    fn buffer_data(&self, context: &WebGl2RenderingContext) {
        unsafe {
            let buffer_view = Float32Array::view(&self.data);
            Self::buffer_data_with_object(context, &buffer_view);
        }
    }

    fn buffer_data_with_object(context: &WebGl2RenderingContext, buffer_view: &js_sys::Object) {
        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            buffer_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    fn vertex_attrib_pointer(&self, context: &WebGl2RenderingContext, location: u32) {
        context.vertex_attrib_pointer_with_i32(
            location,
            self.kind.size,
            self.kind.gl_type,
            false,
            0,
            0,
        );
    }

    pub fn apply_matrix_mut(&mut self, matrix: &Mat4) {
        let default_vec4 = glm::vec4(0.0, 0.0, 0.0, 1.0);
        let size = self.kind.size.try_into().unwrap();
        let get_elem = |base: usize, offset: usize| {
            if offset < size {
                self.data[base + offset]
            } else {
                default_vec4[offset]
            }
        };
        let mut new_data = Vec::with_capacity(self.data.len());
        for i in (0..self.data.len()).step_by(size) {
            let new_vec4 = glm::vec4(
                get_elem(i, 0),
                get_elem(i, 1),
                get_elem(i, 2),
                get_elem(i, 3),
            );
            let new_vec4 = matrix * new_vec4;
            for j in 0..size {
                new_data.push(new_vec4[j]);
            }
        }
        self.data = new_data;
    }

    pub fn concat_mut(&mut self, other: &AttributeData) -> Result<()> {
        if self.kind == other.kind {
            self.data.extend(other.data.iter());
            self.count += other.count;
            Ok(())
        } else {
            Err(anyhow!(
                "Cannot concat attribute {:?} and {:?}",
                self.kind,
                other.kind
            ))
        }
    }
}

impl<const N: usize, const K: usize> From<&[[f32; N]; K]> for AttributeData {
    fn from(data: &[[f32; N]; K]) -> Self {
        fn flatten_array<T: Clone, const N: usize, const K: usize>(data: &[[T; N]; K]) -> Vec<T> {
            data.iter().flat_map(|item| item.to_vec()).collect()
        }
        Self::new_with_flat_array(flatten_array(data), N, data.len())
    }
}

impl<const N: usize> From<&Vec<[f32; N]>> for AttributeData {
    fn from(data: &Vec<[f32; N]>) -> Self {
        fn flatten_array<T: Clone, const N: usize>(data: &[[T; N]]) -> Vec<T> {
            data.iter().flat_map(|item| item.to_vec()).collect()
        }
        Self::new_with_flat_array(flatten_array(data), N, data.len())
    }
}

impl<const N: usize> From<&Vec<SVector<f32, N>>> for AttributeData {
    fn from(data: &Vec<SVector<f32, N>>) -> Self {
        fn flatten_vector<T: Copy, const N: usize>(data: &[SVector<T, N>]) -> Vec<T> {
            data.iter()
                .flat_map(|item| item.iter().copied().collect::<Vec<T>>())
                .collect()
        }
        Self::new_with_flat_array(flatten_vector(data), N, data.len())
    }
}

impl From<&Vec<Color>> for AttributeData {
    fn from(data: &Vec<Color>) -> Self {
        fn flatten_color(data: &[Color]) -> Vec<f32> {
            data.iter().flat_map(|item| item.to_rgba_vec()).collect()
        }
        Self::new_with_flat_array(flatten_color(data), 4, data.len())
    }
}

impl<const N: usize> From<&[Color; N]> for AttributeData {
    fn from(data: &[Color; N]) -> Self {
        fn flatten_color<const N: usize>(data: &[Color; N]) -> Vec<f32> {
            data.iter().flat_map(|item| item.to_rgba_vec()).collect()
        }
        Self::new_with_flat_array(flatten_color(data), 4, data.len())
    }
}

pub struct Attribute {
    data: AttributeData,
    buffer: WebGlBuffer,
}

impl Attribute {
    pub fn new_with_data(
        context: &WebGl2RenderingContext,
        data: AttributeData,
    ) -> Result<Attribute> {
        let buffer = gl::create_buffer(context)?;

        let attribute = Attribute { data, buffer };
        attribute.upload_data(context);
        Ok(attribute)
    }

    pub fn count(&self) -> i32 {
        self.data.count
    }

    fn upload_data(&self, context: &WebGl2RenderingContext) {
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.buffer));
        self.data.buffer_data(context);
    }

    pub fn associate_variable(
        &self,
        context: &WebGl2RenderingContext,
        program: &WebGlProgram,
        variable: &str,
    ) {
        if let Some(location) = gl::get_attrib_location(context, program, variable) {
            context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.buffer));
            self.data.vertex_attrib_pointer(context, location);
            context.enable_vertex_attrib_array(location);
        }
    }

    pub fn apply_matrix_mut(&mut self, context: &WebGl2RenderingContext, matrix: &Mat4) {
        self.data.apply_matrix_mut(matrix);
        self.upload_data(context);
    }

    pub fn concat_mut(
        &mut self,
        context: &WebGl2RenderingContext,
        other: &Attribute,
    ) -> Result<()> {
        self.data.concat_mut(&other.data)?;
        self.upload_data(context);
        Ok(())
    }
}
