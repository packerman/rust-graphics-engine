use super::data::Gltf;

#[derive(Debug, Clone)]
pub struct GltfStatistics {
    pub accessor_count: usize,
    pub buffer_count: usize,
    pub buffer_byte_length: u32,
    pub buffer_view_count: usize,
    pub camera_count: usize,
    material_count: usize,
    pub mesh_count: usize,
    pub primitive_count: usize,
    pub node_count: usize,
    pub scene_count: usize,
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
        }
    }
}
