use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anyhow::{anyhow, Result};
use glm::{Qua, Vec3};
use js_sys::ArrayBuffer;
use web_sys::WebGl2RenderingContext;

use crate::gltf::{
    core::{
        camera::Camera,
        geometry::{Mesh, Primitive},
        scene::{Node, Scene},
        storage::{Accessor, Buffer, BufferView},
    },
    material,
    program::Program,
};

use super::data;

pub fn build_buffers(buffers: &[data::Buffer], array_buffers: Vec<ArrayBuffer>) -> Vec<Rc<Buffer>> {
    array_buffers
        .into_iter()
        .enumerate()
        .map(|(i, array_buffer)| Buffer::new(array_buffer, buffers[i].byte_length))
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

pub fn build_cameras(cameras: &[data::Camera]) -> Vec<Rc<RefCell<Camera>>> {
    cameras
        .iter()
        .map(|camera| match camera.camera_type.as_str() {
            "perspective" => {
                let perspective = camera
                    .perspective
                    .as_ref()
                    .expect("Missing perspective camera structure");
                Camera::perspective(
                    perspective.aspect_ratio.unwrap_or(1.0),
                    perspective.y_fov,
                    perspective.z_near,
                    perspective.z_far,
                    camera.name.clone(),
                )
            }
            "orthographic" => {
                let orthographic = camera
                    .orthographic
                    .as_ref()
                    .expect("Missing orthographic camera structure");
                Camera::orthographic(
                    orthographic.x_mag,
                    orthographic.y_mag,
                    orthographic.z_near,
                    orthographic.z_far,
                    camera.name.clone(),
                )
            }
            _ => panic!("Unknown camera type: {}", camera.camera_type),
        })
        .map(RefCell::new)
        .map(Rc::new)
        .collect()
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
const DEFAULT_ROTATION: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub fn build_nodes(
    gltf_nodes: &[data::Node],
    meshes: &[Rc<Mesh>],
    cameras: &[Rc<RefCell<Camera>>],
) -> Vec<Rc<Node>> {
    let nodes: Vec<_> = gltf_nodes
        .iter()
        .map(|node| {
            let translation = node.translation.unwrap_or(DEFAULT_TRANSLATION);
            let translation = glm::translation(&Vec3::from(translation));
            let rotation = node.rotation.unwrap_or(DEFAULT_ROTATION);
            let rotation = glm::quat_to_mat4(&Qua::from(rotation));
            let transform = translation * rotation;
            Node::new(
                transform,
                node.mesh.map(|index| self::get_rc_u32(meshes, index)),
                node.camera.map(|index| self::get_rc_u32(cameras, index)),
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