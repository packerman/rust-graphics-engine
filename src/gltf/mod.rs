use std::{collections::HashMap, hash::Hash, rc::Rc};

use anyhow::Result;

use url::Url;
use web_sys::WebGl2RenderingContext;

use self::core::{Accessor, Buffer, BufferView, Mesh, Node, Primitive, Root, Scene};

pub mod core;
pub mod material;
pub mod read;
pub mod validate;

pub async fn load(context: &WebGl2RenderingContext, uri: &str) -> Result<Root> {
    let gltf = self::read::fetch_gltf(uri).await?;
    let base_uri = Url::parse(uri)?;
    let buffer_data = self::read::fetch_buffers(&base_uri, gltf.buffers.as_deref()).await?;
    let buffers = self::map_vector(buffer_data, Buffer::new);
    let buffer_views = self::try_map_optional_slice(gltf.buffer_views.as_deref(), |buffer_view| {
        let buffer = self::get_ref(&buffers, buffer_view.buffer);
        BufferView::new(
            context,
            buffer,
            buffer_view.byte_offset,
            buffer_view.byte_length,
            buffer_view.byte_stride,
            buffer_view.target,
        )
    })?;
    let accessors = self::map_optional_slice(gltf.accessors.as_deref(), |accessor| {
        let buffer_view = accessor
            .buffer_view
            .map(|index| self::get_ref(&buffer_views, index));
        let min = &accessor.min;
        let max = &accessor.max;
        Accessor::new(
            buffer_view,
            accessor.byte_offset,
            accessor.component_type,
            accessor.count,
            get_size(&accessor.accessor_type),
            min.clone(),
            max.clone(),
            accessor.normalized,
        )
    });
    let material = Rc::new(material::basic(context)?);
    let meshes = self::try_map_optional_slice(gltf.meshes.as_deref(), |mesh| {
        let primitives: Result<Vec<_>> = mesh
            .primitives
            .iter()
            .map(|primitive| {
                let attributes = self::map_values(&primitive.attributes, |index| {
                    self::get_ref(&accessors, *index)
                });
                Primitive::new(context, attributes, Rc::clone(&material), primitive.mode)
            })
            .collect();
        Ok(Mesh::new(primitives?))
    })?;
    let nodes = self::map_optional_slice(gltf.nodes.as_deref(), |node| {
        Node::new(node.mesh.map(|index| self::get_ref(&meshes, index)))
    });
    let scenes: Vec<_> = gltf
        .scenes
        .unwrap_or_default()
        .into_iter()
        .map(|scene| {
            Scene::new(
                scene
                    .nodes
                    .unwrap_or_default()
                    .iter()
                    .map(|index| self::get_ref(&nodes, *index))
                    .collect(),
            )
        })
        .collect();
    Ok(Root::new(scenes, gltf.scene.map(|index| index as usize)))
}

fn get_size(type_name: &str) -> i32 {
    match type_name {
        "VEC3" => 3,
        _ => panic!("Unknown type: {}", type_name),
    }
}

fn map_vector<T, F, S>(source: Vec<T>, f: F) -> Vec<Rc<S>>
where
    F: Fn(T) -> S,
{
    source.into_iter().map(|t| Rc::new(f(t))).collect()
}

fn map_slice<T, F, S>(source: &[T], f: F) -> Vec<Rc<S>>
where
    F: Fn(&T) -> S,
{
    source.iter().map(|t| Rc::new(f(t))).collect()
}

fn map_optional_slice<T, F, S>(source: Option<&[T]>, f: F) -> Vec<Rc<S>>
where
    F: Fn(&T) -> S,
{
    self::map_slice(source.unwrap_or_default(), f)
}

fn try_map_slice<T, F, S>(source: &[T], f: F) -> Result<Vec<Rc<S>>>
where
    F: Fn(&T) -> Result<S>,
{
    source.iter().map(|t| f(t).map(Rc::new)).collect()
}

fn try_map_optional_slice<T, F, S>(source: Option<&[T]>, f: F) -> Result<Vec<Rc<S>>>
where
    F: Fn(&T) -> Result<S>,
{
    self::try_map_slice(source.unwrap_or_default(), f)
}

fn map_values<K, T, F, S>(hash_map: &HashMap<K, T>, f: F) -> HashMap<K, S>
where
    F: Fn(&T) -> S,
    K: Clone + Eq + Hash,
{
    hash_map.iter().map(|(k, v)| (k.clone(), f(v))).collect()
}

fn get_ref<T>(slice: &[Rc<T>], index: u32) -> Rc<T> {
    Rc::clone(&slice[index as usize])
}
