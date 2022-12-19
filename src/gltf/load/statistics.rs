use std::primitive;

use super::data::Gltf;

#[derive(Debug, Clone)]
pub struct GltfStatistics {
    pub accessor_count: usize,
    pub buffer_count: usize,
    pub buffer_byte_length: u32,
    pub buffer_view_count: usize,
    pub camera_count: usize,
    pub material_count: usize,
    pub mesh_count: usize,
    pub primitive_count: usize,
    pub node_count: usize,
    pub scene_count: usize,
    pub vertex_count: u32,
    pub index_count: u32,
}

impl From<&Gltf> for GltfStatistics {
    fn from(gltf: &Gltf) -> Self {
        GltfStatistics {
            accessor_count: gltf.accessors.as_ref().map(Vec::len).unwrap_or_default(),
            buffer_count: gltf.buffers.as_ref().map(Vec::len).unwrap_or_default(),
            buffer_byte_length: gltf
                .buffers
                .iter()
                .flatten()
                .map(|buffer| buffer.byte_length)
                .sum(),
            buffer_view_count: gltf.buffer_views.as_ref().map(Vec::len).unwrap_or_default(),
            camera_count: gltf.cameras.as_ref().map(Vec::len).unwrap_or_default(),
            material_count: gltf.materials.as_ref().map(Vec::len).unwrap_or_default(),
            mesh_count: gltf.meshes.as_ref().map(Vec::len).unwrap_or_default(),
            primitive_count: gltf
                .meshes
                .iter()
                .flatten()
                .map(|mesh| mesh.primitives.len())
                .sum(),
            node_count: gltf.nodes.as_ref().map(Vec::len).unwrap_or_default(),
            scene_count: gltf.scenes.as_ref().map(Vec::len).unwrap_or_default(),
            vertex_count: gltf
                .meshes
                .iter()
                .flatten()
                .flat_map(|mesh| &mesh.primitives)
                .map(|primitive| {
                    primitive
                        .attributes
                        .get("POSITION")
                        .cloned()
                        .and_then(|index| {
                            gltf.accessors
                                .as_ref()
                                .and_then(|accessors| accessors.get(index as usize))
                        })
                        .map(|accessor| accessor.count as u32)
                        .unwrap_or_default()
                })
                .sum(),
            index_count: gltf
                .meshes
                .iter()
                .flatten()
                .flat_map(|mesh| &mesh.primitives)
                .map(|primitive| {
                    primitive
                        .indices
                        .and_then(|index| {
                            gltf.accessors
                                .as_ref()
                                .and_then(|accessors| accessors.get(index as usize))
                                .map(|accessor| accessor.count as u32)
                        })
                        .unwrap_or_default()
                })
                .sum(),
        }
    }
}
