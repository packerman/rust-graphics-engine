use std::collections::HashMap;

use serde::Deserialize;

use web_sys::WebGl2RenderingContext;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accessor {
    pub buffer_view: Option<u32>,
    #[serde(default)]
    pub byte_offset: u32,
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
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub copyright: Option<String>,
    pub generator: Option<String>,
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
pub struct Camera {
    pub orthographic: Option<Orthographic>,
    pub perspective: Option<Perspective>,
    #[serde(rename = "type")]
    pub camera_type: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct Perspective {
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
    pub images: Option<Vec<Image>>,
    pub materials: Option<Vec<Material>>,
    pub meshes: Option<Vec<Mesh>>,
    pub nodes: Option<Vec<Node>>,
    pub samplers: Option<Vec<Sampler>>,
    pub scene: Option<u32>,
    pub scenes: Option<Vec<Scene>>,
    pub textures: Option<Vec<Texture>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub uri: Option<String>,
    pub mime_type: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Material {
    pub name: Option<String>,
    #[serde(default)]
    pub pbr_metallic_roughness: PbrMetallicRoughness,
    #[serde(default)]
    pub double_sided: bool,
    #[serde(default)]
    pub emissive_factor: [f32; 3],
    pub normal_texture: Option<NormalTextureInfo>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PbrMetallicRoughness {
    #[serde(default = "PbrMetallicRoughness::default_base_color_factor")]
    pub base_color_factor: [f32; 4],
    pub base_color_texture: Option<TextureInfo>,
    #[serde(default = "PbrMetallicRoughness::default_metallic_factor")]
    pub metallic_factor: f32,
    #[serde(default = "PbrMetallicRoughness::default_roughness_factor")]
    pub roughness_factor: f32,
    pub metallic_roughness_texture: Option<TextureInfo>,
}

impl PbrMetallicRoughness {
    pub fn default_base_color_factor() -> [f32; 4] {
        [1.0, 1.0, 1.0, 1.0]
    }

    pub fn default_metallic_factor() -> f32 {
        1.0
    }

    pub fn default_roughness_factor() -> f32 {
        1.0
    }
}

impl Default for PbrMetallicRoughness {
    fn default() -> Self {
        Self {
            base_color_factor: Self::default_base_color_factor(),
            base_color_texture: None,
            metallic_factor: Self::default_metallic_factor(),
            roughness_factor: Self::default_roughness_factor(),
            metallic_roughness_texture: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NormalTextureInfo {
    pub index: u32,
    #[serde(default)]
    pub tex_coord: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextureInfo {
    pub index: u32,
    #[serde(default)]
    pub tex_coord: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mesh {
    pub primitives: Vec<Primitive>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Primitive {
    pub attributes: HashMap<String, u32>,
    pub indices: Option<u32>,
    pub material: Option<u32>,
    #[serde(default = "Primitive::default_mode")]
    pub mode: u32,
}

impl Primitive {
    fn default_mode() -> u32 {
        WebGl2RenderingContext::TRIANGLES
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub camera: Option<u32>,
    pub children: Option<Vec<u32>>,
    pub matrix: Option<[f32; 16]>,
    pub mesh: Option<u32>,
    pub translation: Option<[f32; 3]>,
    pub rotation: Option<[f32; 4]>,
    pub scale: Option<[f32; 3]>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sampler {
    pub mag_filter: Option<i32>,
    pub min_filter: Option<i32>,
    #[serde(default = "Sampler::default_wrap_s")]
    pub wrap_s: i32,
    #[serde(default = "Sampler::default_wrap_t")]
    pub wrap_t: i32,
}

impl Sampler {
    fn default_wrap_s() -> i32 {
        WebGl2RenderingContext::REPEAT as i32
    }

    fn default_wrap_t() -> i32 {
        WebGl2RenderingContext::REPEAT as i32
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scene {
    pub name: Option<String>,
    pub nodes: Option<Vec<u32>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Texture {
    pub sampler: Option<u32>,
    pub source: Option<u32>,
}
