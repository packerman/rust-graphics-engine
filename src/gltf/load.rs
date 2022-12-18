use std::rc::Rc;

use anyhow::Result;
use url::Url;
use web_sys::WebGl2RenderingContext;

use crate::gltf::{core::renderer::Renderer, load::statistics::GltfStatistics, util::coll};

use super::{
    core::{camera::Camera, scene::Scene, storage::Buffer, Root},
    util::shared_ref::SharedRef,
};

pub mod build;
pub mod data;
pub mod fetch;
pub mod statistics;

pub async fn load<'a>(context: &WebGl2RenderingContext, uri: &str) -> Result<Root> {
    let gltf = fetch::fetch_gltf(uri).await?;
    debug!("{:#?}", gltf.asset);
    debug!("{:#?}", GltfStatistics::from(&gltf));
    let base_uri = Url::parse(uri)?;
    let buffers =
        self::load_buffers(&base_uri, coll::flatten_optional_vector(&gltf.buffers)).await?;
    let cameras = build::build_cameras(coll::flatten_optional_vector(&gltf.cameras));
    let scenes = self::load_scenes(context, &gltf, &buffers, &cameras)?;
    let renderer = Renderer::initialize(context);
    Ok(Root::initialize(
        cameras,
        scenes,
        gltf.scene.map(|index| index as usize),
        renderer,
    ))
}

async fn load_buffers(base_uri: &Url, buffers: Vec<&data::Buffer>) -> Result<Vec<Rc<Buffer>>> {
    let array_buffers = fetch::fetch_buffers(base_uri, &buffers).await?;
    Ok(build::build_buffers(buffers, array_buffers))
}

fn load_scenes(
    context: &WebGl2RenderingContext,
    gltf: &data::Gltf,
    buffers: &[Rc<Buffer>],
    cameras: &[SharedRef<Camera>],
) -> Result<Vec<Scene>> {
    let buffer_views =
        build::build_buffer_views(coll::flatten_optional_vector(&gltf.buffer_views), buffers)?;
    let accessors = build::build_accessors(
        context,
        coll::flatten_optional_vector(&gltf.accessors),
        &buffer_views,
    )?;

    let materials =
        build::build_materials(context, coll::flatten_optional_vector(&gltf.materials))?;
    let meshes = build::build_meshes(
        context,
        coll::flatten_optional_vector(&gltf.meshes),
        &accessors,
        &materials,
    )?;
    let nodes = build::build_nodes(coll::flatten_optional_vector(&gltf.nodes), &meshes, cameras);
    Ok(build::build_scenes(
        coll::flatten_optional_vector(&gltf.scenes),
        &nodes,
    ))
}
