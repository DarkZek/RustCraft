use crate::helpers::{from_bevy_vec3, global_to_local_position};
use crate::systems::asset::AssetService;
use crate::systems::chunk::ChunkSystem;
use crate::systems::physics::raycasts::do_raycast;
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use crate::game::blocks::states::BlockStates;
use crate::game::inventory::Inventory;
use crate::systems::chunk::builder::{RerenderChunkFlag, RerenderChunkFlagContext};
use crate::systems::physics::aabb::Aabb;
use rc_networking::constants::{UserId, CHUNK_SIZE};
use rc_networking::protocol::clientbound::block_update::BlockUpdate;
use rc_networking::protocol::Protocol;
use rc_networking::types::SendPacket;

pub fn mouse_interaction(
    mouse_button_input: Res<Input<MouseButton>>,
    mut commands: Commands,
    camera: Query<&Transform, With<Camera>>,
    mut chunks: ResMut<ChunkSystem>,
    mut assets: ResMut<AssetService>,
    mut networking: EventWriter<SendPacket>,
    inventory: Res<Inventory>,
    mut lines: ResMut<DebugLines>,
    blocks: Res<BlockStates>,
    mut rerender_chunks: EventWriter<RerenderChunkFlag>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let camera_pos = camera.get_single().unwrap();

    let look = camera_pos.rotation * Vec3::new(0.0, 0.0, -1.0);

    let cast = do_raycast(
        from_bevy_vec3(camera_pos.translation),
        from_bevy_vec3(look),
        15.0,
        &chunks,
        &blocks,
    );

    if cast.is_none() {
        return;
    }

    let ray = cast.unwrap();

    // Locate chunk
    let (chunk_loc, inner_loc) = global_to_local_position(ray.block);

    // Try find chunk
    if let Some(mut chunk) = chunks.chunks.get_mut(&chunk_loc) {
        // Highlight selected block
        let block = blocks.get_block(chunk.world[inner_loc.x][inner_loc.y][inner_loc.z] as usize);

        Aabb::draw_lines(&block.bounding_boxes, ray.block.cast::<f32>(), &mut lines);

        if mouse_button_input.just_pressed(MouseButton::Left) {
            // Found chunk! Update block
            chunk.world[inner_loc.x][inner_loc.y][inner_loc.z] = 0;

            // Rerender
            rerender_chunks.send(RerenderChunkFlag {
                chunk: chunk_loc,
                context: RerenderChunkFlagContext::Surrounding,
            });

            info!(
                "Destroyed [{}, {}, {}]",
                ray.block.x as usize % CHUNK_SIZE,
                ray.block.y as usize % CHUNK_SIZE,
                ray.block.z as usize % CHUNK_SIZE
            );

            // Send network update
            networking.send(SendPacket(
                Protocol::BlockUpdate(BlockUpdate::new(0, ray.block.x, ray.block.y, ray.block.z)),
                UserId(0),
            ))
        }
    }
    if mouse_button_input.just_pressed(MouseButton::Right) {
        if let Some(block_type) = inventory.selected_block_id() {
            let pos = ray.block + ray.normal;

            // Locate chunk
            let (chunk_loc, inner_loc) = global_to_local_position(pos);

            // Try find chunk
            if let Some(mut chunk) = chunks.chunks.get_mut(&chunk_loc) {
                // Found chunk! Update block
                chunk.world[inner_loc.x][inner_loc.y][inner_loc.z] = block_type;

                // Rerender
                rerender_chunks.send(RerenderChunkFlag {
                    chunk: chunk_loc,
                    context: RerenderChunkFlagContext::Surrounding,
                });

                info!(
                    "Updated [{}, {}, {}]",
                    ray.block.x, ray.block.y, ray.block.z
                );
            } else {
                // Create chunk data
                let mut chunk = [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

                // Set block
                chunk[inner_loc.x][inner_loc.y][inner_loc.z] = block_type;

                // Create chunk
                chunks.create_chunk(
                    chunk_loc,
                    chunk,
                    &mut commands,
                    &mut assets,
                    &mut rerender_chunks,
                    &mut meshes,
                );
            }

            // Send network update
            networking.send(SendPacket(
                Protocol::BlockUpdate(BlockUpdate::new(block_type, pos.x, pos.y, pos.z)),
                UserId(0),
            ))
        }
    }
}
