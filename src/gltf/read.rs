use std::collections::HashMap;

use anyhow::{anyhow, Result};
use js_sys::ArrayBuffer;
use serde::Deserialize;
use url::Url;
use web_sys::WebGl2RenderingContext;

use crate::core::web;

use super::validate::{self, Validate};

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
    version: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Buffer {
    pub uri: Option<String>,
    byte_length: u32,
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
}

#[derive(Debug, Clone, Deserialize)]
pub struct Node {
    pub mesh: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Scene {
    pub nodes: Option<Vec<u32>>,
}

pub async fn fetch_gltf(uri: &str) -> Result<Gltf> {
    serde_wasm_bindgen::from_value(web::fetch_json(uri).await?)
        .map_err(|error| anyhow!("Error while fetching glTF from {}: {:#?}", uri, error))
}

pub async fn fetch_buffers(base_url: &Url, buffers: Option<&[Buffer]>) -> Result<Vec<ArrayBuffer>> {
    if let Some(buffers) = buffers {
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
    } else {
        Ok(vec![])
    }
}
