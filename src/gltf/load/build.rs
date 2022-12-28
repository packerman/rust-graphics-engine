use std::{collections::HashMap, rc::Rc};

use anyhow::{anyhow, Result};
use glm::{Qua, Vec3, Vec4};
use js_sys::ArrayBuffer;
use web_sys::{HtmlImageElement, WebGl2RenderingContext};

use crate::{
    core::{
        camera::Camera,
        geometry::{Mesh, Primitive},
        material::{AlphaMode, Material, TextureRef},
        scene::{Node, Scene},
        storage::{Accessor, AccessorProperties, AccessorType, Buffer, BufferView},
        texture_data::{Image, Sampler, Texture},
    },
    gltf::{
        material::TestMaterial,
        util::shared_ref::{self, SharedRef},
    },
};

use super::data;

pub fn build_buffers(
    buffers: Vec<&data::Buffer>,
    array_buffers: Vec<ArrayBuffer>,
) -> Vec<Rc<Buffer>> {
    array_buffers
        .into_iter()
        .enumerate()
        .map(|(i, array_buffer)| Buffer::new(array_buffer, buffers[i].byte_length as usize))
        .map(Rc::new)
        .collect()
}

pub fn build_buffer_views(
    buffer_views: Vec<&data::BufferView>,
    buffers: &[Rc<Buffer>],
) -> Result<Vec<Rc<BufferView>>> {
    buffer_views
        .into_iter()
        .map(|buffer_view| {
            let buffer = self::get_rc_by_u32(buffers, buffer_view.buffer);
            BufferView::new(
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
    context: &WebGl2RenderingContext,
    accessors: Vec<&data::Accessor>,
    buffer_views: &[Rc<BufferView>],
) -> Result<Vec<Rc<Accessor>>> {
    accessors
        .into_iter()
        .map(|accessor| {
            let buffer_view = accessor
                .buffer_view
                .map(|index| self::get_rc_by_u32(buffer_views, index));
            let min = &accessor.min;
            let max = &accessor.max;
            Accessor::initialize(
                context,
                buffer_view,
                AccessorProperties {
                    byte_offset: accessor.byte_offset,
                    component_type: accessor.component_type,
                    count: accessor.count,
                    accessor_type: get_size(&accessor.accessor_type)?,
                    min: min.clone(),
                    max: max.clone(),
                    normalized: accessor.normalized,
                },
            )
            .map(Rc::new)
        })
        .collect()
}

fn get_size(accessor_type: &str) -> Result<AccessorType> {
    match accessor_type {
        "SCALAR" => Ok(AccessorType::scalar()),
        "VEC2" => Ok(AccessorType::vec(2)),
        "VEC3" => Ok(AccessorType::vec(3)),
        "VEC4" => Ok(AccessorType::vec(4)),
        _ => Err(anyhow!("Unknown type: {}", accessor_type)),
    }
}

pub fn build_cameras(cameras: Vec<&data::Camera>) -> Vec<SharedRef<Camera>> {
    cameras
        .into_iter()
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
        .map(shared_ref::strong)
        .collect()
}

pub fn build_images(
    images: Vec<&data::Image>,
    html_images: Vec<HtmlImageElement>,
) -> Vec<Rc<Image>> {
    html_images
        .into_iter()
        .enumerate()
        .map(|(index, html_image)| {
            Image::new(
                html_image,
                images.get(index).and_then(|image| image.name.clone()),
                images.get(index).and_then(|image| image.mime_type.clone()),
            )
        })
        .map(Rc::new)
        .collect()
}

pub fn build_materials(
    context: &WebGl2RenderingContext,
    materials: Vec<&data::Material>,
    textures: &[Rc<Texture>],
) -> Result<Vec<Rc<Material>>> {
    fn build_alpha_mode(material: &data::Material) -> Result<AlphaMode> {
        match material.alpha_mode.as_str() {
            "OPAQUE" => Ok(AlphaMode::Opaque),
            "MASK" => Ok(AlphaMode::Mask {
                cutoff: material.alpha_cutoff,
            }),
            "BLEND" => Ok(AlphaMode::Blend),
            _ => Err(anyhow!("Unknown alpha mode: {}", material.alpha_mode)),
        }
    }

    materials
        .into_iter()
        .map(|material| {
            let alpha_mode = build_alpha_mode(material)?;
            Material::initialize(
                context,
                material.name.clone(),
                material.double_sided,
                Rc::new(TestMaterial {
                    base_color_factor: Vec4::from(
                        material.pbr_metallic_roughness.base_color_factor,
                    ),
                    base_color_texture: material
                        .pbr_metallic_roughness
                        .base_color_texture
                        .as_ref()
                        .map(|texture_info| {
                            TextureRef::new(
                                self::get_rc_by_u32(textures, texture_info.index),
                                texture_info.tex_coord,
                            )
                        }),
                    ..Default::default()
                }),
                alpha_mode,
            )
            .map(Rc::new)
        })
        .collect()
}

pub fn build_meshes(
    context: &WebGl2RenderingContext,
    meshes: Vec<&data::Mesh>,
    accessors: &[Rc<Accessor>],
    materials: &[Rc<Material>],
) -> Result<Vec<Rc<Mesh>>> {
    meshes
        .into_iter()
        .map(|mesh| {
            let primitives =
                self::build_primitives(context, &mesh.primitives, accessors, materials)?;
            let mesh = Mesh::new(primitives, mesh.name.as_ref().map(String::from));
            Ok(Rc::new(mesh))
        })
        .collect()
}

fn build_primitives(
    context: &WebGl2RenderingContext,
    primitives: &[data::Primitive],
    accessors: &[Rc<Accessor>],
    materials: &[Rc<Material>],
) -> Result<Vec<Primitive>> {
    primitives
        .iter()
        .map(|primitive| {
            let attributes = self::build_attributes(&primitive.attributes, accessors);
            let indices = primitive
                .indices
                .map(|index| self::get_rc_by_u32(accessors, index));
            Primitive::new(
                context,
                attributes,
                indices,
                if let Some(index) = primitive.material {
                    self::get_rc_by_u32(materials, index)
                } else {
                    self::default_material(context)?
                },
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
        .map(|(attribute, index)| {
            (
                String::from(attribute),
                self::get_rc_by_u32(accessors, *index),
            )
        })
        .collect()
}

const DEFAULT_TRANSLATION: [f32; 3] = [0.0, 0.0, 0.0];
const DEFAULT_ROTATION: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const DEFAULT_SCALE: [f32; 3] = [1.0, 1.0, 1.0];

pub fn build_nodes(
    gltf_nodes: Vec<&data::Node>,
    meshes: &[Rc<Mesh>],
    cameras: &[SharedRef<Camera>],
) -> Vec<SharedRef<Node>> {
    let nodes: Vec<_> = gltf_nodes
        .iter()
        .map(|node| {
            let transform = if let Some(matrix) = node.matrix {
                glm::make_mat4(&matrix)
            } else {
                let translation =
                    glm::translation(&Vec3::from(node.translation.unwrap_or(DEFAULT_TRANSLATION)));
                let rotation =
                    glm::quat_to_mat4(&Qua::from(node.rotation.unwrap_or(DEFAULT_ROTATION)));
                let scale = glm::scaling(&Vec3::from(node.scale.unwrap_or(DEFAULT_SCALE)));
                translation * rotation * scale
            };
            Node::new(
                transform,
                node.mesh.map(|index| self::get_rc_by_u32(meshes, index)),
                node.camera
                    .map(|index| self::get_cloned_by_u32(cameras, index)),
                node.name.clone(),
            )
        })
        .collect();
    for (i, gltf_node) in gltf_nodes.iter().enumerate() {
        for child_index in gltf_node.children.iter().flatten() {
            let node = &nodes[i];
            let child = self::get_cloned_by_u32(&nodes, *child_index);
            node.borrow_mut().add_child(child);
        }
    }
    nodes
}

pub fn build_samplers(samplers: Vec<&data::Sampler>) -> Result<Vec<Rc<Sampler>>> {
    samplers
        .iter()
        .map(|sampler| {
            Sampler::new(
                sampler.mag_filter,
                sampler.min_filter,
                sampler.wrap_s,
                sampler.wrap_t,
            )
            .map(Rc::new)
        })
        .collect()
}

pub fn build_textures(
    context: &WebGl2RenderingContext,
    textures: Vec<&data::Texture>,
    samplers: &[Rc<Sampler>],
    images: &[Rc<Image>],
) -> Result<Vec<Rc<Texture>>> {
    textures
        .into_iter()
        .map(|texture| {
            let sampler = texture
                .sampler
                .map(|index| self::get_rc_by_u32(samplers, index))
                .unwrap_or_default();
            let source = texture
                .source
                .map(|index| self::get_rc_by_u32(images, index))
                .expect("Expected source image in texture");
            Texture::initialize(context, sampler, source).map(Rc::new)
        })
        .collect()
}

pub fn build_scenes(scenes: Vec<&data::Scene>, nodes: &[SharedRef<Node>]) -> Vec<Scene> {
    scenes
        .into_iter()
        .map(|scene| {
            Scene::new(
                scene
                    .nodes
                    .iter()
                    .flatten()
                    .map(|index| self::get_cloned_by_u32(nodes, *index))
                    .collect(),
            )
        })
        .collect()
}

fn default_material(context: &WebGl2RenderingContext) -> Result<Rc<Material>> {
    Material::initialize(
        context,
        None,
        false,
        Rc::<TestMaterial>::default(),
        AlphaMode::default(),
    )
    .map(Rc::new)
}

fn get_rc_by_u32<T>(slice: &[Rc<T>], index: u32) -> Rc<T> {
    Rc::clone(&slice[index as usize])
}

fn get_cloned_by_u32<T: Clone>(slice: &[T], index: u32) -> T {
    slice[index as usize].clone()
}
