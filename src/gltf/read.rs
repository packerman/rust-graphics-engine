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
pub struct Accessor {
    pub buffer_view: Option<Integer>,
    #[serde(default)]
    pub byte_offset: i32,
    pub component_type: u32,
    pub count: Integer,
    #[serde(rename = "type")]
    pub accessor_type: String,
    pub min: Option<Vec<Number>>,
    pub max: Option<Vec<Number>>,
    #[serde(default)]
    pub normalized: bool,
}

#[derive(Debug, Deserialize)]
pub struct Asset {
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
    #[serde(default)]
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
    pub asset: Asset,
    pub accessors: Vec<Accessor>,
    pub buffers: Vec<Buffer>,
    pub buffer_views: Vec<BufferView>,
    pub meshes: Vec<Mesh>,
    pub nodes: Vec<Node>,
    pub scene: Option<Integer>,
    pub scenes: Vec<Scene>,
}

#[derive(Debug, Deserialize)]
pub struct Mesh {
    pub primitives: Vec<Primitive>,
}

#[derive(Debug, Deserialize)]
pub struct Primitive {
    pub attributes: HashMap<String, u32>,
}

#[derive(Debug, Deserialize)]
pub struct Node {
    pub mesh: Option<Integer>,
}

#[derive(Debug, Deserialize)]
pub struct Scene {
    pub nodes: Vec<Integer>,
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
