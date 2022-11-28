use std::collections::HashMap;

use anyhow::{anyhow, Result};
use serde::Deserialize;

use crate::core::web;

type Integer = i32;
type Number = f64;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Accessor {
    buffer_view: Option<Integer>,
    #[serde(default = "default_byte_offset")]
    byte_offset: Integer,
    component_type: Integer,
    count: Integer,
    #[serde(rename = "type")]
    accessor_type: String,
    min: Option<Vec<Number>>,
    max: Option<Vec<Number>>,
}

fn default_byte_offset() -> Integer {
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
    byte_length: Integer,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BufferView {
    buffer: Integer,
    #[serde(default = "default_byte_offset")]
    byte_offset: Integer,
    byte_length: Option<Integer>,
    target: Option<Integer>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gltf {
    asset: Asset,
    accessors: Vec<Accessor>,
    buffers: Vec<Buffer>,
    buffer_views: Vec<BufferView>,
    meshes: Vec<Mesh>,
    nodes: Vec<Node>,
    scene: Option<Integer>,
    scenes: Vec<Scene>,
}

#[derive(Debug, Deserialize)]
struct Mesh {
    primitives: Vec<Primitive>,
}

#[derive(Debug, Deserialize)]
struct Primitive {
    attributes: HashMap<String, Integer>,
}

#[derive(Debug, Deserialize)]
struct Node {
    mesh: Option<Integer>,
}

#[derive(Debug, Deserialize)]
struct Scene {
    nodes: Vec<Integer>,
}

pub async fn fetch_gltf(uri: &str) -> Result<Gltf> {
    serde_wasm_bindgen::from_value(web::fetch_json(uri).await?)
        .map_err(|error| anyhow!("Error while fetching glTF from {}: {:#?}", uri, error))
}
