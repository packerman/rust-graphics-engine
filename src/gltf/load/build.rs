use std::{collections::HashMap, rc::Rc};

use anyhow::{anyhow, Result};
use js_sys::ArrayBuffer;
use web_sys::WebGl2RenderingContext;

use crate::{
    core::math::matrix,
    gltf::{
        core::{
            geometry::{Mesh, Primitive},
            scene::{Node, Scene},
            storage::{Accessor, Buffer, BufferView},
        },
        material,
        program::Program,
    },
};

use super::data;

pub fn build_buffers(buffers: &[data::Buffer], array_buffers: Vec<ArrayBuffer>) -> Vec<Rc<Buffer>> {
    buffers
        .iter()
        .enumerate()
        .map(|(i, buffer)| Buffer::new(array_buffers[i].to_owned(), buffer.byte_length))
        .map(Rc::new)
        .collect()
}

pub fn build_buffer_views(
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

pub fn build_accessors(
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

pub fn build_meshes(
    context: &WebGl2RenderingContext,
    meshes: &[data::Mesh],
    accessors: &[Rc<Accessor>],
) -> Result<Vec<Rc<Mesh>>> {
    let material = Rc::new(material::basic(context)?);
    meshes
        .iter()
        .map(|mesh| {
            let primitives =
                self::build_primitives(context, &mesh.primitives, accessors, &material)?;
            let mesh = Mesh::new(primitives);
            Ok(Rc::new(mesh))
        })
        .collect()
}

fn build_primitives(
    context: &WebGl2RenderingContext,
    primitives: &[data::Primitive],
    accessors: &[Rc<Accessor>],
    material: &Rc<Program>,
) -> Result<Vec<Primitive>> {
    primitives
        .iter()
        .map(|primitive| {
            let attributes = self::build_attributes(&primitive.attributes, accessors);
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

fn build_attributes(
    attributes: &HashMap<String, u32>,
    accessors: &[Rc<Accessor>],
) -> HashMap<String, Rc<Accessor>> {
    attributes
        .iter()
        .map(|(attribute, index)| (String::from(attribute), self::get_rc_u32(accessors, *index)))
        .collect()
}

const DEFAULT_TRANSLATION: [f32; 3] = [0.0, 0.0, 0.0];

pub fn build_nodes(gltf_nodes: &[data::Node], meshes: &[Rc<Mesh>]) -> Vec<Rc<Node>> {
    let nodes: Vec<_> = gltf_nodes
        .iter()
        .map(|node| {
            let translation = node.translation.unwrap_or(DEFAULT_TRANSLATION);
            let transform = matrix::translation(translation[0], translation[1], translation[2]);
            Node::new(
                transform,
                node.mesh.map(|index| self::get_rc_u32(meshes, index)),
            )
        })
        .collect();
    for (i, gltf_node) in gltf_nodes.iter().enumerate() {
        for child_index in gltf_node.children.iter().flatten() {
            let node = &nodes[i];
            let child = self::get_rc_u32(&nodes, *child_index);
            node.add_child(child);
        }
    }
    nodes
}

pub fn build_scenes(scenes: &[data::Scene], nodes: &[Rc<Node>]) -> Vec<Scene> {
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
