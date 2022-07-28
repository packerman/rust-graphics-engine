use anyhow::{Ok, Result};
use js_sys::Float32Array;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram};

use super::gl::{create_buffer, get_attrib_location};

pub struct DataType {
    size: i32,
    base_type: u32,
}

impl DataType {
    const fn new(size: i32, base_type: u32) -> DataType {
        DataType { size, base_type }
    }

    pub const VEC3: DataType = DataType::new(3, WebGl2RenderingContext::FLOAT);
}

pub type VertexData<'a, const N: usize> = &'a [[f32; N]];

pub struct Attribute<'a, const N: usize> {
    data_type: &'a DataType,
    data: VertexData<'a, N>,
    buffer: WebGlBuffer,
}

impl<'a, const N: usize> Attribute<'a, N> {
    pub fn new_with_data(
        context: &WebGl2RenderingContext,
        data_type: &'a DataType,
        data: VertexData<'a, N>,
    ) -> Result<Attribute<'a, N>> {
        let buffer = create_buffer(context)?;

        let attribute = Attribute {
            data_type,
            data,
            buffer,
        };
        attribute.upload_data(context);
        Ok(attribute)
    }

    pub fn upload_data(&self, context: &WebGl2RenderingContext) {
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.buffer));
        let flat_data = flatten_data(self.data);
        unsafe {
            let buffer_view = Float32Array::view(&flat_data);
            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &buffer_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
    }

    pub fn associate_variable(
        &self,
        context: &WebGl2RenderingContext,
        program: &WebGlProgram,
        variable: &str,
    ) -> Result<()> {
        let location = get_attrib_location(context, program, variable)?;
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

fn flatten_data<'a, const N: usize>(data: VertexData<'a, N>) -> Vec<f32> {
    data.into_iter().flat_map(|item| item.to_vec()).collect()
}
