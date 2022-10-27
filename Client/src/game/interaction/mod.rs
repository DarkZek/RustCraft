use crate::helpers::{from_bevy_vec3, global_to_local_position};
use crate::services::asset::AssetService;
use crate::services::chunk::ChunkService;
use crate::services::networking::transport::packet::SendPacket;
use crate::services::physics::raycasts::do_raycast;
use crate::{
    info, Assets, Camera, ChunkData, Commands, Entity, EulerRot, EventWriter, Input, Mesh,
    MouseButton, Query, RerenderChunkFlag, Res, ResMut, StandardMaterial, Transform, Vec3, With,
};
use nalgebra::Vector3;
use rustcraft_protocol::constants::CHUNK_SIZE;
use rustcraft_protocol::protocol::clientbound::block_update::BlockUpdate;
use rustcraft_protocol::protocol::Protocol;
use std::ops::Mul;

pub fn mouse_interaction(
    mouse_button_input: Res<Input<MouseButton>>,
    mut commands: Commands,
    camera: Query<&Transform, With<Camera>>,
    mut chunks: ResMut<ChunkService>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut assets: ResMut<AssetService>,
    mut networking: EventWriter<SendPacket>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left)
        && !mouse_button_input.just_pressed(MouseButton::Right)
    {
        return;
    }

    let camera_pos = camera.get_single().unwrap();

    let look = camera_pos.rotation * Vec3::new(0.0, 0.0, -1.0);

    let cast = do_raycast(
        from_bevy_vec3(camera_pos.translation),
        from_bevy_vec3(look),
        15.0,
        &chunks,
        &mut commands,
        &mut meshes,
    );

    if cast.is_none() {
        return;
    }

    let ray = cast.unwrap();

    if mouse_button_input.just_pressed(MouseButton::Left) {
        // Locate chunk
        let (chunk_loc, inner_loc) = global_to_local_position(ray.block);

        // Try find chunk
        if let Some(mut chunk) = chunks.chunks.get_mut(&chunk_loc) {
            // Found chunk! Update block
            chunk.world[inner_loc.x][inner_loc.y][inner_loc.z] = 0;

            // Rerender
            commands
                .entity(chunk.entity)
                .insert(RerenderChunkFlag { chunk: chunk_loc });

            info!(
                "Destroyed [{}, {}, {}]",
                ray.block.x as usize % CHUNK_SIZE,
                ray.block.y as usize % CHUNK_SIZE,
                ray.block.z as usize % CHUNK_SIZE
            );
        }

        // Send network update
        networking.send(SendPacket(Protocol::BlockUpdate(BlockUpdate::new(
            0,
            ray.block.x,
            ray.block.y,
            ray.block.z,
        ))))
    } else {
        let pos = ray.block + ray.normal;

        // Locate chunk
        let (chunk_loc, inner_loc) = global_to_local_position(pos);

        // Try find chunk
        if let Some(mut chunk) = chunks.chunks.get_mut(&chunk_loc) {
            // Found chunk! Update block
            chunk.world[inner_loc.x][inner_loc.y][inner_loc.z] = 1;

            // Rerender
            commands
                .entity(chunk.entity)
                .insert(RerenderChunkFlag { chunk: chunk_loc });

            info!(
                "Updated [{}, {}, {}]",
                ray.block.x as usize % CHUNK_SIZE,
                ray.block.y as usize % CHUNK_SIZE,
                ray.block.z as usize % CHUNK_SIZE
            );
        } else {
            // Create chunk data
            let mut chunk = [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

            // Set block
            chunk[inner_loc.x][inner_loc.y][inner_loc.z] = 1;

            // Create chunk
            chunks.create_chunk(chunk_loc, chunk, &mut commands, &mut assets, &mut meshes);
        }

        // Send network update
        networking.send(SendPacket(Protocol::BlockUpdate(BlockUpdate::new(
            1,
            ray.block.x,
            ray.block.y,
            ray.block.z,
        ))))
    }
}
