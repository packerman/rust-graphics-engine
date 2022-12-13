use anyhow::Result;
use url::Url;
use web_sys::WebGl2RenderingContext;

use crate::gltf::load::data::GltfStatistics;

use super::core::Root;

pub mod build;
pub mod data;
pub mod fetch;

pub async fn load(context: &WebGl2RenderingContext, uri: &str) -> Result<Root> {
    let gltf = fetch::fetch_gltf(uri).await?;
    debug!("{:#?}", gltf.asset);
    debug!("{:#?}", GltfStatistics::from(&gltf));
    let base_uri = Url::parse(uri)?;
    let buffers = fetch::fetch_buffers(&base_uri, &gltf.buffers.unwrap_or_default()).await?;
    let buffer_views =
        build::load_buffer_views(context, &gltf.buffer_views.unwrap_or_default(), &buffers)?;
    let accessors = build::load_accessors(&gltf.accessors.unwrap_or_default(), &buffer_views)?;
    let meshes = build::load_meshes(context, &gltf.meshes.unwrap_or_default(), &accessors)?;
    let nodes = build::load_nodes(&gltf.nodes.unwrap_or_default(), &meshes);
    let scenes = build::load_scenes(&gltf.scenes.unwrap_or_default(), &nodes);
    Ok(Root::new(scenes, gltf.scene.map(|index| index as usize)))
}
