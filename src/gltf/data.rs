use std::collections::HashMap;

use anyhow::{anyhow, Result};

use serde::Deserialize;

use web_sys::WebGl2RenderingContext;

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
