use anyhow::{anyhow, Result};
use js_sys::ArrayBuffer;
use url::Url;
use web_sys::WebGl2RenderingContext;

use crate::core::gl;

use self::{
    core::GlBuffer,
    read::{Accessor, BufferView, Mesh},
};

pub mod core;
pub mod material;
pub mod read;
pub mod validate;

pub async fn load(context: &WebGl2RenderingContext, uri: &str) -> Result<()> {
    let gltf = self::read::fetch_gltf(uri).await?;
    let base_uri = Url::parse(uri)?;
    let buffer_data = self::read::fetch_buffers(&base_uri, &gltf.buffers).await?;
    let buffers = self::create_gl_buffers(context, &buffer_data, &gltf.buffer_views)?;
    Ok(())
}

fn process_meshes(
    context: &WebGl2RenderingContext,
    accessors: &[Accessor],
    meshes: &[Mesh],
) -> Result<()> {
    let material = material::basic(context)?;
    meshes.iter().for_each(|mesh| {
        mesh.primitives.iter().map(|primitive| -> Result<()> {
            let vertex_array = gl::create_vertex_array(context)?;
            context.bind_vertex_array(Some(&vertex_array));
            for (attribute, accessor_index) in primitive.attributes.iter() {
                let attribute = format!("a_{}", attribute.to_lowercase());
                if let Some(location) =
                    gl::get_attrib_location(context, material.program(), &attribute)
                {
                    let accessor = &accessors[*accessor_index as usize];
                    context.vertex_attrib_pointer_with_i32(
                        location,
                        get_size(&accessor.accessor_type),
                        accessor.component_type,
                        accessor.normalized,
                        0,
                        accessor.byte_offset,
                    );
                    context.enable_vertex_attrib_array(location);
                }
            }
            context.bind_vertex_array(None);
            Ok(())
        });
    });
    Ok(())
}

fn get_size(type_name: &str) -> i32 {
    match type_name {
        "VEC3" => 3,
        _ => panic!("Unknown type: {}", type_name),
    }
}

fn create_gl_buffers(
    context: &WebGl2RenderingContext,
    buffers: &[ArrayBuffer],
    buffer_views: &[BufferView],
) -> Result<Vec<GlBuffer>> {
    buffer_views
        .iter()
        .map(|buffer_view| {
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
