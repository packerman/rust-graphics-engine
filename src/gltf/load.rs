use std::{collections::HashMap, rc::Rc};

use anyhow::{anyhow, Result};
use url::Url;
use web_sys::WebGl2RenderingContext;

use crate::{core::material::Material, gltf::data::GltfStatistics};

use super::{
    core::{Accessor, Buffer, BufferView, Mesh, Node, Primitive, Root, Scene},
    data, fetch, material,
};

pub async fn load(context: &WebGl2RenderingContext, uri: &str) -> Result<Root> {
    let gltf = fetch::fetch_gltf(uri).await?;
    debug!("{:#?}", gltf.asset);
    debug!("{:#?}", GltfStatistics::from(&gltf));
    let base_uri = Url::parse(uri)?;
    let buffers = fetch::fetch_buffers(&base_uri, &gltf.buffers.unwrap_or_default()).await?;
    let buffer_views =
        self::load_buffer_views(context, &gltf.buffer_views.unwrap_or_default(), &buffers)?;
    let accessors = self::load_accessors(&gltf.accessors.unwrap_or_default(), &buffer_views)?;
    let meshes = self::load_meshes(context, &gltf.meshes.unwrap_or_default(), &accessors)?;
    let nodes = self::load_nodes(&gltf.nodes.unwrap_or_default(), &meshes);
    let scenes = self::load_scenes(&gltf.scenes.unwrap_or_default(), &nodes);
    Ok(Root::new(scenes, gltf.scene.map(|index| index as usize)))
}

fn load_buffer_views(
    context: &WebGl2RenderingContext,
    buffer_views: &[data::BufferView],
    buffers: &[Rc<Buffer>],
) -> Result<Vec<Rc<BufferView>>> {
    buffer_views
        .iter()
        .map(|buffer_view| {
            let buffer = self::get_rc_u32(buffers, buffer_view.buffer);
            BufferView::new(
                context,
                buffer,
                buffer_view.byte_offset,
                buffer_view.byte_length,
                buffer_view.byte_stride,
                buffer_view.target,
            )
            .map(Rc::new)
        })
        .collect()
}

fn load_accessors(
    accessors: &[data::Accessor],
    buffer_views: &[Rc<BufferView>],
) -> Result<Vec<Rc<Accessor>>> {
    accessors
        .iter()
        .map(|accessor| {
            let buffer_view = accessor
                .buffer_view
                .map(|index| self::get_rc_u32(buffer_views, index));
            let min = &accessor.min;
            let max = &accessor.max;
            Accessor::new(
                buffer_view,
                accessor.byte_offset,
                accessor.component_type,
                accessor.count,
                get_size(&accessor.accessor_type)?,
                min.clone(),
                max.clone(),
                accessor.normalized,
            )
            .map(Rc::new)
        })
        .collect()
}

fn get_size(accessor_type: &str) -> Result<i32> {
    match accessor_type {
        "VEC3" => Ok(3),
        "SCALAR" => Ok(1),
        _ => Err(anyhow!("Unknown type: {}", accessor_type)),
    }
}

fn load_meshes(
    context: &WebGl2RenderingContext,
    meshes: &[data::Mesh],
    accessors: &[Rc<Accessor>],
) -> Result<Vec<Rc<Mesh>>> {
    let material = Rc::new(material::basic(context)?);
    meshes
        .iter()
        .map(|mesh| {
            let primitives =
                self::load_primitives(context, &mesh.primitives, accessors, &material)?;
            let mesh = Mesh::new(primitives);
            Ok(Rc::new(mesh))
        })
        .collect()
}

fn load_primitives(
    context: &WebGl2RenderingContext,
    primitives: &[data::Primitive],
    accessors: &[Rc<Accessor>],
    material: &Rc<Material>,
) -> Result<Vec<Primitive>> {
    primitives
        .iter()
        .map(|primitive| {
            let attributes = self::load_attributes(&primitive.attributes, accessors);
            let indices = primitive
                .indices
                .map(|index| self::get_rc_u32(accessors, index));
            Primitive::new(
                context,
                attributes,
                indices,
                Rc::clone(material),
                primitive.mode,
            )
        })
        .collect()
}

fn load_attributes(
    attributes: &HashMap<String, u32>,
    accessors: &[Rc<Accessor>],
) -> HashMap<String, Rc<Accessor>> {
    attributes
        .iter()
        .map(|(attribute, index)| (String::from(attribute), self::get_rc_u32(accessors, *index)))
        .collect()
}

fn load_nodes(nodes: &[data::Node], meshes: &[Rc<Mesh>]) -> Vec<Rc<Node>> {
    nodes
        .iter()
        .map(|node| Node::new(node.mesh.map(|index| self::get_rc_u32(meshes, index))))
        .map(Rc::new)
        .collect()
}

fn load_scenes(scenes: &[data::Scene], nodes: &[Rc<Node>]) -> Vec<Scene> {
    scenes
        .iter()
        .map(|scene| {
            Scene::new(
                scene
                    .nodes
                    .iter()
                    .flatten()
                    .map(|index| self::get_rc_u32(nodes, *index))
                    .collect(),
            )
        })
        .collect()
}

fn get_rc_u32<T>(slice: &[Rc<T>], index: u32) -> Rc<T> {
    Rc::clone(&slice[index as usize])
}
