use anyhow::{Ok, Result};
use js_sys::Float32Array;
use na::SVector;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram};

use super::{color::Color, gl};

pub struct DataType {
    size: i32,
    base_type: u32,
}

impl DataType {
    const fn new(size: i32, base_type: u32) -> DataType {
        DataType { size, base_type }
    }
}
pub trait AttributeData {
    fn buffer_data(&self, context: &WebGl2RenderingContext);
}

impl AttributeData for Vec<f32> {
    fn buffer_data(&self, context: &WebGl2RenderingContext) {
        fn buffer_data(context: &WebGl2RenderingContext, buffer_view: &js_sys::Object) {
            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                buffer_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
        unsafe {
            let buffer_view = Float32Array::view(self);
            buffer_data(context, &buffer_view);
        }
    }
}

pub struct Attribute {
    data_type: DataType,
    data: Box<dyn AttributeData>,
    buffer: WebGlBuffer,
    pub vertex_count: usize,
}

impl Attribute {
    pub fn from_array<const N: usize>(
        context: &WebGl2RenderingContext,
        data: &[[f32; N]],
    ) -> Result<Attribute> {
        fn flatten_array<T: Clone, const N: usize>(data: &[[T; N]]) -> Vec<T> {
            data.iter().flat_map(|item| item.to_vec()).collect()
        }
        let flat_data = Box::new(flatten_array(data));
        Self::from_flat_array(context, flat_data, N, data.len())
    }

    pub fn from_vector_array<const N: usize>(
        context: &WebGl2RenderingContext,
        data: &[SVector<f32, N>],
    ) -> Result<Attribute> {
        fn flatten_vector<T: Copy, const N: usize>(data: &[SVector<T, N>]) -> Vec<T> {
            data.iter()
                .flat_map(|item| item.iter().copied().collect::<Vec<T>>())
                .collect()
        }
        let flat_data = Box::new(flatten_vector(data));
        Self::from_flat_array(context, flat_data, N, data.len())
    }

    pub fn from_rgb_color_array(
        context: &WebGl2RenderingContext,
        data: &[Color],
    ) -> Result<Attribute> {
        fn flatten_color(data: &[Color]) -> Vec<f32> {
            data.iter().flat_map(|item| item.to_rgb_vec()).collect()
        }
        let flat_data = Box::new(flatten_color(data));
        Self::from_flat_array(context, flat_data, 3, data.len())
    }

    fn new_with_data(
        context: &WebGl2RenderingContext,
        data_type: DataType,
        data: Box<dyn AttributeData>,
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

    fn from_flat_array(
        context: &WebGl2RenderingContext,
        data: Box<dyn AttributeData>,
        size: usize,
        length: usize,
    ) -> Result<Attribute> {
        Self::new_with_data(
            context,
            DataType::new(size.try_into().unwrap(), WebGl2RenderingContext::FLOAT),
            data,
            length,
        )
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
}
