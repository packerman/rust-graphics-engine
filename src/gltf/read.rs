use std::collections::HashMap;

use anyhow::{anyhow, Result};
use js_sys::ArrayBuffer;
use serde::Deserialize;
use url::Url;
use web_sys::WebGl2RenderingContext;

use crate::core::web;

use super::validate::{self, Validate};

type Integer = i32;
type Number = f64;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Accessor {
    buffer_view: Option<Integer>,
    #[serde(default = "default_byte_offset")]
    byte_offset: u32,
    component_type: Integer,
    count: Integer,
    #[serde(rename = "type")]
    accessor_type: String,
    min: Option<Vec<Number>>,
    max: Option<Vec<Number>>,
}

fn default_byte_offset() -> u32 {
    0
}

#[derive(Debug, Deserialize)]
struct Asset {
    version: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Buffer {
    pub uri: Option<String>,
    byte_length: Integer,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BufferView {
    pub buffer: Integer,
    #[serde(default = "default_byte_offset")]
    pub byte_offset: u32,
    pub byte_length: Option<u32>,
    pub target: Option<u32>,
}

const TARGETS: [u32; 2] = [
    WebGl2RenderingContext::ARRAY_BUFFER,
    WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
];

impl Validate for BufferView {
    fn validate(&self) -> Result<()> {
        validate::optional(&self.target, |target| {
            validate::contains(target, &TARGETS, |value| {
                anyhow!("Unkown target: {}", value)
            })
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gltf {
    asset: Asset,
    accessors: Vec<Accessor>,
    pub buffers: Vec<Buffer>,
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

pub async fn fetch_buffers(base_url: &Url, buffers: &[Buffer]) -> Result<Vec<ArrayBuffer>> {
    let mut array_buffers = Vec::with_capacity(buffers.len());
    for (i, buffer) in buffers.iter().enumerate() {
        let relative_uri = buffer
            .uri
            .as_ref()
            .ok_or_else(|| anyhow!("Undefined url in buffer[{}]", i))?;
        let url = base_url.join(relative_uri)?;
        array_buffers.push(web::fetch_array_buffer(url.as_str()).await?);
    }
    Ok(array_buffers)
}
