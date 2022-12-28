use std::rc::Rc;

use anyhow::Result;
use url::Url;
use web_sys::WebGl2RenderingContext;

use crate::{
    core::{camera::Camera, scene::Scene, storage::Buffer, texture_data::Image},
    gltf::{core::Root, load::statistics::GltfStatistics, util::coll},
};

use super::util::shared_ref::SharedRef;

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
    let images = self::load_images(&base_uri, coll::flatten_optional_vector(&gltf.images)).await?;
    let cameras = build::build_cameras(coll::flatten_optional_vector(&gltf.cameras));
    let scenes = self::load_scenes(context, &gltf, &buffers, &images, &cameras)?;
    Ok(Root::initialize(
        context,
        cameras,
        scenes,
        gltf.scene.map(|index| index as usize),
    ))
}

async fn load_buffers(base_uri: &Url, buffers: Vec<&data::Buffer>) -> Result<Vec<Rc<Buffer>>> {
    let array_buffers = fetch::fetch_buffers(base_uri, &buffers).await?;
    Ok(build::build_buffers(buffers, array_buffers))
}

async fn load_images(base_uri: &Url, images: Vec<&data::Image>) -> Result<Vec<Rc<Image>>> {
    let html_images = fetch::fetch_images(base_uri, &images).await?;
    Ok(build::build_images(images, html_images))
}

fn load_scenes(
    context: &WebGl2RenderingContext,
    gltf: &data::Gltf,
    buffers: &[Rc<Buffer>],
    images: &[Rc<Image>],
    cameras: &[SharedRef<Camera>],
) -> Result<Vec<Scene>> {
    let buffer_views =
        build::build_buffer_views(coll::flatten_optional_vector(&gltf.buffer_views), buffers)?;
    let accessors = build::build_accessors(
        context,
        coll::flatten_optional_vector(&gltf.accessors),
        &buffer_views,
    )?;
    let samplers = build::build_samplers(coll::flatten_optional_vector(&gltf.samplers))?;
    let textures = build::build_textures(
        context,
        coll::flatten_optional_vector(&gltf.textures),
        &samplers,
        images,
    )?;
    let materials = build::build_materials(
        context,
        coll::flatten_optional_vector(&gltf.materials),
        &textures,
    )?;
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
