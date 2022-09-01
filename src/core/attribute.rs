use anyhow::{anyhow, Result};
use glm::Mat4;
use js_sys::Float32Array;
use na::SVector;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram};

use super::{color::Color, gl};

#[derive(Debug, PartialEq, Eq)]
pub struct DataType {
    size: i32,
    base_type: u32,
}

impl DataType {
    const fn new(size: i32, base_type: u32) -> DataType {
        DataType { size, base_type }
    }
}

pub struct Attribute {
    data_type: DataType,
    data: Vec<f32>,
    buffer: WebGlBuffer,
    pub vertex_count: usize,
}

impl Attribute {
    pub fn with_array<const N: usize>(
        context: &WebGl2RenderingContext,
        data: &[[f32; N]],
    ) -> Result<Attribute> {
        fn flatten_array<T: Clone, const N: usize>(data: &[[T; N]]) -> Vec<T> {
            data.iter().flat_map(|item| item.to_vec()).collect()
        }
        Self::with_flat_array(context, flatten_array(data), N, data.len())
    }

    pub fn with_vector_array<const N: usize>(
        context: &WebGl2RenderingContext,
        data: &[SVector<f32, N>],
    ) -> Result<Attribute> {
        fn flatten_vector<T: Copy, const N: usize>(data: &[SVector<T, N>]) -> Vec<T> {
            data.iter()
                .flat_map(|item| item.iter().copied().collect::<Vec<T>>())
                .collect()
        }
        Self::with_flat_array(context, flatten_vector(data), N, data.len())
    }

    pub fn with_rgb_color_array(
        context: &WebGl2RenderingContext,
        data: &[Color],
    ) -> Result<Attribute> {
        fn flatten_color(data: &[Color]) -> Vec<f32> {
            data.iter().flat_map(|item| item.to_rgb_vec()).collect()
        }
        Self::with_flat_array(context, flatten_color(data), 3, data.len())
    }

    pub fn with_rgba_color_array(
        context: &WebGl2RenderingContext,
        data: &[Color],
    ) -> Result<Attribute> {
        fn flatten_color(data: &[Color]) -> Vec<f32> {
            data.iter().flat_map(|item| item.to_rgba_vec()).collect()
        }
        Self::with_flat_array(context, flatten_color(data), 4, data.len())
    }

    fn with_flat_array(
        context: &WebGl2RenderingContext,
        data: Vec<f32>,
        size: usize,
        length: usize,
    ) -> Result<Attribute> {
        Attribute::new_with_data(
            context,
            DataType::new(size.try_into().unwrap(), WebGl2RenderingContext::FLOAT),
            data,
            length,
        )
    }

    fn new_with_data(
        context: &WebGl2RenderingContext,
        data_type: DataType,
        data: Vec<f32>,
        vertex_count: usize,
    ) -> Result<Attribute> {
        let buffer = gl::create_buffer(context)?;

        let attribute = Attribute {
            data_type,
            data,
            buffer,
            vertex_count,
        };
        attribute.upload_data(context);
        Ok(attribute)
    }

    fn upload_data(&self, context: &WebGl2RenderingContext) {
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.buffer));
        unsafe {
            let buffer_view = Float32Array::view(&self.data);
            Self::buffer_data(context, &buffer_view);
        }
    }

    fn buffer_data(context: &WebGl2RenderingContext, buffer_view: &js_sys::Object) {
        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            buffer_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    pub fn associate_variable(
        &self,
        context: &WebGl2RenderingContext,
        program: &WebGlProgram,
        variable: &str,
    ) -> Result<()> {
        let location = gl::get_attrib_location(context, program, variable)?;
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.buffer));
        context.vertex_attrib_pointer_with_i32(
            location,
            self.data_type.size,
            self.data_type.base_type,
            false,
            0,
            0,
        );
        context.enable_vertex_attrib_array(location);
        Ok(())
    }

    pub fn apply_matrix_mut(&mut self, context: &WebGl2RenderingContext, matrix: &Mat4) {
        let default_vec4 = glm::vec4(0.0, 0.0, 0.0, 1.0);
        let size = self.data_type.size.try_into().unwrap();
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
        self.upload_data(context);
    }

    pub fn concat_mut(
        &mut self,
        context: &WebGl2RenderingContext,
        other: &Attribute,
    ) -> Result<()> {
        if self.data_type == other.data_type {
            self.data.extend(other.data.iter());
            self.vertex_count += other.vertex_count;
            self.upload_data(context);
            Ok(())
        } else {
            Err(anyhow!(
                "Cannot concat attribute {:?} and {:?}",
                self.data_type,
                other.data_type
            ))
        }
    }
}
