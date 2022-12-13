use std::rc::Rc;

use anyhow::{anyhow, Result};

use url::Url;

use crate::{core::web, gltf::core::storage::Buffer};

use super::data::{self, Gltf};

pub async fn fetch_gltf(uri: &str) -> Result<Gltf> {
    serde_wasm_bindgen::from_value(web::fetch_json(uri).await?)
        .map_err(|error| anyhow!("Error while fetching glTF from {}: {:#?}", uri, error))
}

pub async fn fetch_buffers(base_url: &Url, buffers: &[data::Buffer]) -> Result<Vec<Rc<Buffer>>> {
    let mut result = Vec::with_capacity(buffers.len());
    for (i, buffer) in buffers.iter().enumerate() {
        let relative_uri = buffer
            .uri
            .as_ref()
            .ok_or_else(|| anyhow!("Undefined url in buffer[{}]", i))?;
        let url = base_url.join(relative_uri)?;
        let array_buffer = web::fetch_array_buffer(url.as_str()).await?;
        let buffer = Buffer::new(array_buffer, buffer.byte_length);
        result.push(Rc::new(buffer));
    }
    Ok(result)
}
