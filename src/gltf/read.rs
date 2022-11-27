use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Accessor {
    buffer_view: Option<usize>,
    #[serde(default = "default_byte_offset")]
    byte_offset: usize,
    component_type: usize,
    count: usize,
    #[serde(rename = "type")]
    accessor_type: String,
    min: Option<Vec<f64>>,
    max: Option<Vec<f64>>,
}

fn default_byte_offset() -> usize {
    0
}

#[derive(Debug, Deserialize)]
struct Asset {
    version: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Buffer {
    uri: Option<String>,
    byte_length: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BufferView {
    buffer: usize,
    #[serde(default = "default_byte_offset")]
    byte_offset: usize,
    byte_length: Option<usize>,
    target: Option<usize>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Gltf {
    asset: Asset,
    accessors: Vec<Accessor>,
    buffers: Vec<Buffer>,
    buffer_views: Vec<BufferView>,
    meshes: Vec<Mesh>,
    nodes: Vec<Node>,
    scene: Option<usize>,
    scenes: Vec<Scene>,
}

#[derive(Debug, Deserialize)]
struct Mesh {
    primitives: Vec<Primitive>,
}

#[derive(Debug, Deserialize)]
struct Primitive {
    attributes: HashMap<String, usize>,
}

#[derive(Debug, Deserialize)]
struct Node {
    mesh: Option<usize>,
}

#[derive(Debug, Deserialize)]
struct Scene {
    nodes: Vec<usize>,
}
