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
    pub min: Option<Vec<f32>>,
    pub max: Option<Vec<f32>>,
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
pub struct Camera {
    pub orthographic: Option<Orthographic>,
    pub perspective: Option<Perspective>,
    #[serde(rename = "type")]
    pub camera_type: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Orthographic {
    #[serde(rename = "xmag")]
    pub x_mag: f32,
    #[serde(rename = "ymag")]
    pub y_mag: f32,
    #[serde(rename = "zfar")]
    pub z_far: f32,
    #[serde(rename = "znear")]
    pub z_near: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Perspective {
    #[serde(rename = "aspectRatio")]
    pub aspect_ratio: Option<f32>,
    #[serde(rename = "yfov")]
    pub y_fov: f32,
    #[serde(rename = "zfar")]
    pub z_far: Option<f32>,
    #[serde(rename = "znear")]
    pub z_near: f32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gltf {
    pub asset: Asset,
    pub accessors: Option<Vec<Accessor>>,
    pub buffers: Option<Vec<Buffer>>,
    pub buffer_views: Option<Vec<BufferView>>,
    pub cameras: Option<Vec<Camera>>,
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
    pub camera: Option<u32>,
    pub children: Option<Vec<u32>>,
    pub mesh: Option<u32>,
    pub translation: Option<[f32; 3]>,
    pub rotation: Option<[f32; 4]>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Scene {
    pub nodes: Option<Vec<u32>>,
}
