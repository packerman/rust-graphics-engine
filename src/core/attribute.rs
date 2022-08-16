use anyhow::{Ok, Result};
use js_sys::Float32Array;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram};

use super::gl;

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
    fn data_type(&self) -> DataType;
    fn buffer_data(&self, context: &WebGl2RenderingContext);
    fn vertex_count(&self) -> usize;
}

impl<const N: usize, const K: usize> AttributeData for &[[f32; N]; K] {
    fn data_type(&self) -> DataType {
        DataType::new(N.try_into().unwrap(), WebGl2RenderingContext::FLOAT)
    }

    fn buffer_data(&self, context: &WebGl2RenderingContext) {
        let flat_data = flatten_data(self);
        unsafe {
            let buffer_view = Float32Array::view(&flat_data);
            buffer_data(context, &buffer_view);
        }
    }

    fn vertex_count(&self) -> usize {
        self.len()
    }
}

pub struct Attribute {
    data_type: DataType,
    buffer: WebGlBuffer,
    pub vertex_count: usize,
}

impl Attribute {
    pub fn new_with_data<D>(context: &WebGl2RenderingContext, data: D) -> Result<Attribute>
    where
        D: AttributeData,
    {
        let buffer = gl::create_buffer(context)?;

        let attribute = Attribute {
            data_type: data.data_type(),
            buffer,
            vertex_count: data.vertex_count(),
        };
        attribute.upload_data(context, data);
        Ok(attribute)
    }

    fn upload_data<D>(&self, context: &WebGl2RenderingContext, data: D)
    where
        D: AttributeData,
    {
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.buffer));
        data.buffer_data(context);
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

fn flatten_data<T: Clone, const N: usize, const K: usize>(data: &[[T; N]; K]) -> Vec<T> {
    data.into_iter().flat_map(|item| item.to_vec()).collect()
}

fn buffer_data(context: &WebGl2RenderingContext, buffer_view: &js_sys::Object) {
    context.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ARRAY_BUFFER,
        buffer_view,
        WebGl2RenderingContext::STATIC_DRAW,
    );
}
