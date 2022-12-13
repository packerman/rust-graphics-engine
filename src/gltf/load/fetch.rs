use anyhow::{anyhow, Result};

use js_sys::ArrayBuffer;
use url::Url;

use crate::core::web;

use super::data::{self, Gltf};

pub async fn fetch_gltf(uri: &str) -> Result<Gltf> {
    serde_wasm_bindgen::from_value(web::fetch_json(uri).await?)
        .map_err(|error| anyhow!("Error while fetching glTF from {}: {:#?}", uri, error))
}

pub async fn fetch_buffers(base_url: &Url, buffers: &[data::Buffer]) -> Result<Vec<ArrayBuffer>> {
    let mut result = Vec::with_capacity(buffers.len());
    for (i, buffer) in buffers.iter().enumerate() {
        let relative_uri = buffer
            .uri
            .as_ref()
            .ok_or_else(|| anyhow!("Undefined url in buffer[{}]", i))?;
        let url = base_url.join(relative_uri)?;
        let array_buffer = web::fetch_array_buffer(url.as_str()).await?;
        result.push(array_buffer);
    }
    Ok(result)
}
