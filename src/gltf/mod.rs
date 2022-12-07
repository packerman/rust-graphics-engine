use std::collections::HashMap;

use anyhow::{anyhow, Result};
use js_sys::ArrayBuffer;
use url::Url;
use web_sys::WebGl2RenderingContext;

use crate::core::gl;

use self::{
    core::{BufferView, Mesh, Primitive},
    read::{Accessor as GltfAccessor, BufferView as GltfBufferView, Mesh as GltfMesh},
};

pub mod core;
pub mod material;
pub mod read;
pub mod validate;

pub async fn load(context: &WebGl2RenderingContext, uri: &str) -> Result<()> {
    let gltf = self::read::fetch_gltf(uri).await?;
    let base_uri = Url::parse(uri)?;
    let buffer_data = self::read::fetch_buffers(&base_uri, &gltf.buffers).await?;
    let buffer_views = self::create_gl_buffers(context, &buffer_data, &gltf.buffer_views)?;
    let meshes = self::process_meshes(context, &buffer_views, &gltf.accessors, &gltf.meshes)?;
    Ok(())
}

fn process_meshes(
    context: &WebGl2RenderingContext,
    buffer_views: &[BufferView],
    accessors: &[GltfAccessor],
    meshes: &[GltfMesh],
) -> Result<Vec<Mesh>> {
    let material = material::basic(context)?;
    let meshes: Result<Vec<_>> = meshes
        .iter()
        .map(|mesh| -> Result<Mesh> {
            let primitives: Result<Vec<_>> = mesh
                .primitives
                .iter()
                .map(|primitive| -> Result<Primitive> {
                    let vertex_array = gl::create_vertex_array(context)?;
                    context.bind_vertex_array(Some(&vertex_array));
                    let count = get_count(&primitive.attributes, accessors)?;
                    for (attribute, index) in primitive.attributes.iter() {
                        let attribute = format!("a_{}", attribute.to_lowercase());
                        if let Some(location) =
                            gl::get_attrib_location(context, material.program(), &attribute)
                        {
                            let accessor = &accessors[*index as usize];
                            let buffer_view = &buffer_views[accessor
                                .buffer_view
                                .ok_or_else(|| anyhow!("Undefined buffer view"))?
                                as usize];
                            buffer_view.bind(context);
                            context.vertex_attrib_pointer_with_i32(
                                location,
                                get_size(&accessor.accessor_type),
                                accessor.component_type,
                                accessor.normalized,
                                buffer_view.byte_stride,
                                accessor.byte_offset,
                            );
                            context.enable_vertex_attrib_array(location);
                        }
                    }
                    context.bind_vertex_array(None);
                    Ok(Primitive::new(vertex_array, count))
                })
                .collect();
            Ok(Mesh::new(primitives?))
        })
        .collect();
    Ok(meshes?)
}

fn get_size(type_name: &str) -> i32 {
    match type_name {
        "VEC3" => 3,
        _ => panic!("Unknown type: {}", type_name),
    }
}

fn get_count(atttributes: &HashMap<String, u32>, accessors: &[GltfAccessor]) -> Result<i32> {
    let counts: Vec<_> = atttributes
        .values()
        .map(|index| &accessors[*index as usize])
        .map(|accessor| accessor.count)
        .collect();
    if counts.is_empty() {
        Err(anyhow!("Attributes map is empty"))
    } else {
        let count = counts[0];
        if counts.into_iter().all(|value| value == count) {
            Ok(count)
        } else {
            Err(anyhow!("All accessors count have to be equal"))
        }
    }
}

fn create_gl_buffers(
    context: &WebGl2RenderingContext,
    buffers: &[ArrayBuffer],
    buffer_views: &[GltfBufferView],
) -> Result<Vec<BufferView>> {
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
            Ok(BufferView::new(
                object,
                target,
                buffer_view.byte_stride.unwrap_or_default(),
            ))
        })
        .collect()
}
