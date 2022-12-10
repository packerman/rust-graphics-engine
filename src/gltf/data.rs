use std::collections::HashMap;

use serde::Deserialize;

use web_sys::WebGl2RenderingContext;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accessor {
    pub buffer_view: Option<u32>,
    #[serde(default)]
    pub byte_offset: i32,
    pub component_type: u32,
    pub count: i32,
    #[serde(rename = "type")]
    pub accessor_type: String,
    pub min: Option<Vec<f64>>,
    pub max: Option<Vec<f64>>,
    #[serde(default)]
    pub normalized: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Asset {
    pub version: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Buffer {
    pub uri: Option<String>,
    pub byte_length: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BufferView {
    pub buffer: u32,
    #[serde(default)]
    pub byte_offset: u32,
    pub byte_length: u32,
    pub byte_stride: Option<i32>,
    pub target: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gltf {
    pub asset: Asset,
    pub accessors: Option<Vec<Accessor>>,
    pub buffers: Option<Vec<Buffer>>,
    pub buffer_views: Option<Vec<BufferView>>,
    pub meshes: Option<Vec<Mesh>>,
    pub nodes: Option<Vec<Node>>,
    pub scene: Option<u32>,
    pub scenes: Option<Vec<Scene>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Mesh {
    pub primitives: Vec<Primitive>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Primitive {
    pub attributes: HashMap<String, u32>,
    pub indices: Option<u32>,
    #[serde(default = "Primitive::default_mode")]
    pub mode: u32,
}

impl Primitive {
    fn default_mode() -> u32 {
        WebGl2RenderingContext::TRIANGLES
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Node {
    pub mesh: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Scene {
    pub nodes: Option<Vec<u32>>,
}

#[derive(Debug, Clone)]
pub struct GltfStatistics {
    pub accessor_count: usize,
    pub buffer_count: usize,
    pub buffer_byte_length: u32,
    pub buffer_view_count: usize,
    pub mesh_count: usize,
    pub primitive_count: usize,
    pub node_count: usize,
    pub scene_count: usize,
}

impl From<&Gltf> for GltfStatistics {
    fn from(gltf: &Gltf) -> Self {
        GltfStatistics {
            accessor_count: gltf.accessors.as_ref().map(Vec::len).unwrap_or_default(),
            buffer_count: gltf.buffers.as_ref().map(Vec::len).unwrap_or_default(),
            buffer_byte_length: gltf
                .buffers
                .iter()
                .flatten()
                .map(|buffer| buffer.byte_length)
                .sum(),
            buffer_view_count: gltf.buffer_views.as_ref().map(Vec::len).unwrap_or_default(),
            mesh_count: gltf.meshes.as_ref().map(Vec::len).unwrap_or_default(),
            primitive_count: gltf
                .meshes
                .iter()
                .flatten()
                .map(|mesh| mesh.primitives.len())
                .sum(),
            node_count: gltf.nodes.as_ref().map(Vec::len).unwrap_or_default(),
            scene_count: gltf.scenes.as_ref().map(Vec::len).unwrap_or_default(),
        }
    }
}
