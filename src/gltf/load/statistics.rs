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
    pub vertex_count: i32,
    pub index_count: i32,
}

impl From<&Gltf> for GltfStatistics {
    fn from(gltf: &Gltf) -> Self {
        GltfStatistics {
            accessor_count: self::get_count(&gltf.accessors),
            buffer_count: self::get_count(&gltf.buffers),
            buffer_byte_length: gltf
                .buffers
                .iter()
                .flatten()
                .map(|buffer| buffer.byte_length)
                .sum(),
            buffer_view_count: self::get_count(&gltf.buffer_views),
            camera_count: self::get_count(&gltf.cameras),
            material_count: self::get_count(&gltf.materials),
            mesh_count: self::get_count(&gltf.meshes),
            primitive_count: gltf
                .meshes
                .iter()
                .flatten()
                .map(|mesh| mesh.primitives.len())
                .sum(),
            node_count: self::get_count(&gltf.nodes),
            scene_count: self::get_count(&gltf.scenes),
            vertex_count: self::get_vertex_count(&gltf),
            index_count: self::get_index_count(&gltf),
        }
    }
}

fn get_count<T>(vec: &Option<Vec<T>>) -> usize {
    vec.as_ref().map(Vec::len).unwrap_or_default()
}

fn get_vertex_count(gltf: &Gltf) -> i32 {
    gltf.meshes
        .iter()
        .flatten()
        .flat_map(|mesh| &mesh.primitives)
        .map(|primitive| {
            primitive
                .attributes
                .get("POSITION")
                .copied()
                .and_then(|index| {
                    gltf.accessors
                        .as_ref()
                        .and_then(|accessors| accessors.get(index as usize))
                })
                .map(|accessor| accessor.count)
                .unwrap_or_default()
        })
        .sum()
}

fn get_index_count(gltf: &Gltf) -> i32 {
    gltf.meshes
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
                        .map(|accessor| accessor.count)
                })
                .unwrap_or_default()
        })
        .sum()
}
