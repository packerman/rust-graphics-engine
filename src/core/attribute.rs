use anyhow::{anyhow, Result};
use js_sys::Float32Array;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

struct DataType {
    size: usize,
    base_type: usize,
}

type VertexData<'a, const N: usize> = &'a [[f32; N]];

struct Attribute<'a, const N: usize> {
    data_type: &'a DataType,
    data: VertexData<'a, N>,
    buffer: WebGlBuffer,
}

impl<'a, const N: usize> Attribute<'a, N> {
    fn new_with_data(
        context: &WebGl2RenderingContext,
        data_type: &'a DataType,
        data: VertexData<'a, N>,
    ) -> Result<Attribute<'a, N>> {
        let buffer = context
            .create_buffer()
            .ok_or_else(|| anyhow!("Cannot create buffer"))?;

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
        let flat_data = self
            .data
            .into_iter()
            .flat_map(|item| item.to_vec())
            .collect::<Vec<f32>>();
        unsafe {
            let buffer_view = Float32Array::view(&flat_data);
            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &buffer_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
    }
}
