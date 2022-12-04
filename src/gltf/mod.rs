use anyhow::{anyhow, Result};
use js_sys::ArrayBuffer;
use url::Url;
use web_sys::WebGl2RenderingContext;

use crate::core::gl;

use self::{core::GlBuffer, read::BufferView};

pub mod core;
pub mod read;
pub mod validate;

pub async fn load(uri: &str) -> Result<()> {
    let gltf = self::read::fetch_gltf(uri).await?;
    let base_uri = Url::parse(uri)?;
    let buffers = self::read::fetch_buffers(&base_uri, &gltf.buffers).await?;
    Ok(())
}

fn create_gl_buffers(
    context: &WebGl2RenderingContext,
    buffers: &[ArrayBuffer],
    buffer_views: &[BufferView],
) -> Result<Vec<GlBuffer>> {
    buffer_views
        .iter()
        .map(|buffer_view| -> Result<GlBuffer> {
            let object = gl::create_buffer(context)?;
            let target = buffer_view
                .target
                .ok_or_else(|| anyhow!("Target not specified"))?;
            context.bind_buffer(target, Some(&object));
            let data = &buffers[buffer_view.buffer as usize];
            if let Some(byte_length) = buffer_view.byte_length {
                context.buffer_data_with_array_buffer_view_and_src_offset_and_length(
                    target,
                    data,
                    WebGl2RenderingContext::STATIC_DRAW,
                    buffer_view.byte_offset,
                    byte_length,
                );
            } else {
                context.buffer_data_with_array_buffer_view_and_src_offset(
                    target,
                    data,
                    WebGl2RenderingContext::STATIC_DRAW,
                    buffer_view.byte_offset,
                );
            }
            Ok(GlBuffer::new(object, target))
        })
        .collect()
}
