use std::rc::Rc;

use anyhow::Result;
use url::Url;
use web_sys::WebGl2RenderingContext;

use crate::gltf::load::statistics::GltfStatistics;

use super::core::{storage::Buffer, Root};

pub mod build;
pub mod data;
pub mod fetch;
pub mod statistics;

pub async fn load(context: &WebGl2RenderingContext, uri: &str) -> Result<Root> {
    let gltf = fetch::fetch_gltf(uri).await?;
    debug!("{:#?}", gltf.asset);
    debug!("{:#?}", GltfStatistics::from(&gltf));
    let base_uri = Url::parse(uri)?;
    let buffers = self::load_buffers(&base_uri, &gltf.buffers.unwrap_or_default()).await?;
    let buffer_views =
        build::build_buffer_views(context, &gltf.buffer_views.unwrap_or_default(), &buffers)?;
    let accessors = build::build_accessors(&gltf.accessors.unwrap_or_default(), &buffer_views)?;
    let cameras = build::build_cameras(&gltf.cameras.unwrap_or_default());
    let meshes = build::build_meshes(context, &gltf.meshes.unwrap_or_default(), &accessors)?;
    let nodes = build::build_nodes(&gltf.nodes.unwrap_or_default(), &meshes, &cameras);
    let scenes = build::build_scenes(&gltf.scenes.unwrap_or_default(), &nodes);
    Ok(Root::new(
        cameras,
        scenes,
        gltf.scene.map(|index| index as usize),
    ))
}

async fn load_buffers(base_uri: &Url, buffers: &[data::Buffer]) -> Result<Vec<Rc<Buffer>>> {
    let array_buffers = fetch::fetch_buffers(base_uri, buffers).await?;
    Ok(build::build_buffers(buffers, array_buffers))
}
