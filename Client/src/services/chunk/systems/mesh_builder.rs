use crate::game::blocks::BlockStates;
use crate::helpers::from_bevy_vec3;
use crate::services::chunk::data::generate_mesh::UpdateChunkMesh;


use crate::services::chunk::ChunkService;
use crate::Component;
use crate::{
    Assets, Camera, Commands, Entity, Handle, Query, Res, ResMut, Transform, With,
};


use bevy::prelude::Mesh;
use bevy::render::mesh::Indices;
use nalgebra::Vector3;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rustcraft_protocol::constants::CHUNK_SIZE;

#[derive(Component)]
pub struct RerenderChunkFlag {
    pub chunk: Vector3<i32>,
}

pub fn mesh_builder(
    mut commands: Commands,
    flags: Query<(Entity, &RerenderChunkFlag, &Handle<Mesh>)>,
    chunks: Res<ChunkService>,
    mut meshes: ResMut<Assets<Mesh>>,
    camera: Query<&Transform, With<Camera>>,
    block_states: Res<BlockStates>,
) {
    // If there's no chunks to re-render, return
    if flags.is_empty() {
        return;
    }

    let chunks_to_compute = get_closest_chunks(
        &flags.iter().collect(),
        from_bevy_vec3(camera.single().translation),
    );

    #[cfg(not(target_arch = "wasm32"))]
    let iterator = chunks_to_compute.par_iter();
    #[cfg(target_arch = "wasm32")]
    let iterator = chunks_to_compute.iter();

    let updates = iterator
        .map(|(entity, pos, pbr)| {
            // If the data exists
            if let Option::Some(chunk) = chunks.chunks.get(&pos.chunk) {
                assert_eq!(chunk.position, pos.chunk);

                // Generate mesh & gpu buffers
                Some((
                    chunk.generate_mesh(&chunks, &block_states, true),
                    pbr,
                    entity,
                ))
            } else {
                None
            }
        })
        .collect::<Vec<Option<(UpdateChunkMesh, &&Handle<Mesh>, &Entity)>>>();

    for update in updates {
        if let Option::Some((val, pbr, entity)) = update {
            apply_mesh(val, meshes.get_mut(pbr).unwrap());

            commands.entity(*entity).remove::<RerenderChunkFlag>();
        }
    }
}

/// Gets the closest X chunks to batch render
fn get_closest_chunks<'a>(
    entities: &Vec<(Entity, &'a RerenderChunkFlag, &'a Handle<Mesh>)>,
    camera: Vector3<f32>,
) -> Vec<(Entity, &'a RerenderChunkFlag, &'a Handle<Mesh>)> {
    let chunk_size = CHUNK_SIZE as i32;

    let mut chunks = entities.clone();

    chunks.sort_unstable_by(|(_, flag_1, _), (_, flag_2, _)| {
        let chunk_center_1 = Vector3::new(
            (flag_1.chunk.x * chunk_size) as f32 + (chunk_size / 2) as f32,
            (flag_1.chunk.y * chunk_size) as f32 + (chunk_size / 2) as f32,
            (flag_1.chunk.z * chunk_size) as f32 + (chunk_size / 2) as f32,
        );

        let chunk_center_2 = Vector3::new(
            (flag_2.chunk.x * chunk_size) as f32 + (chunk_size / 2) as f32,
            (flag_2.chunk.y * chunk_size) as f32 + (chunk_size / 2) as f32,
            (flag_2.chunk.z * chunk_size) as f32 + (chunk_size / 2) as f32,
        );

        let offset_1 = chunk_center_1 - camera;
        let distance_1: f32 = offset_1.x.abs() + offset_1.y.abs() + offset_1.z.abs();

        let offset_2 = chunk_center_2 - camera;
        let distance_2: f32 = offset_2.x.abs() + offset_2.y.abs() + offset_2.z.abs();

        distance_1.partial_cmp(&distance_2).unwrap()
    });

    // Only render 80 chunks at a time
    chunks.truncate(80);

    chunks
}

fn apply_mesh(update: UpdateChunkMesh, mesh: &mut Mesh) {
    mesh.set_indices(Some(Indices::U32(update.indices)));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, update.positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, update.normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, update.uv_coordinates);
}
