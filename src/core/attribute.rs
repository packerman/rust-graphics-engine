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
}

pub struct Attribute<D> {
    data_type: DataType,
    data: D,
    buffer: WebGlBuffer,
}

impl<D> Attribute<D> {
    pub fn new_with_data(context: &WebGl2RenderingContext, data: D) -> Result<Attribute<D>>
    where
        D: AttributeData,
    {
        let buffer = gl::create_buffer(context)?;

        let attribute = Attribute {
            data_type: data.data_type(),
            data,
            buffer,
        };
        attribute.upload_data(context);
        Ok(attribute)
    }

    pub fn upload_data(&self, context: &WebGl2RenderingContext)
    where
        D: AttributeData,
    {
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
