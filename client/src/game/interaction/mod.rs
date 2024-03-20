pub mod highlight;

use crate::systems::asset::AssetService;
use crate::systems::chunk::ChunkSystem;
use crate::systems::physics::raycasts::do_raycast;
use bevy::prelude::*;

use crate::game::events::DestroyBlockEvent;
use crate::game::inventory::Inventory;
use crate::systems::chunk::builder::{RerenderChunkFlag, RerenderChunkFlagContext};
use rc_shared::constants::UserId;
use rc_networking::protocol::clientbound::block_update::BlockUpdate;
use rc_networking::protocol::Protocol;
use rc_networking::types::SendPacket;
use rc_shared::block::BlockStates;
use rc_shared::helpers::{from_bevy_vec3, global_to_local_position};
use rc_shared::CHUNK_SIZE;

pub fn mouse_interaction(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    camera: Query<&Transform, With<Camera>>,
    assets: Res<AssetService>,
    mut destroy_block_event: EventWriter<DestroyBlockEvent>,
    mut rerender_chunk_event: EventWriter<RerenderChunkFlag>,
    mut chunks: ResMut<ChunkSystem>,
    blocks: Res<BlockStates>,
    mut inventory: ResMut<Inventory>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut send_packet: EventWriter<SendPacket>,
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
    if let Some(chunk) = chunks.chunks.get(&chunk_loc) {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            let block_id = chunk.world[inner_loc.x][inner_loc.y][inner_loc.z];

            destroy_block_event.send(DestroyBlockEvent {
                player_triggered: true,
                position: ray.block,
                block_id,
            });
        }
    }
    if mouse_button_input.just_pressed(MouseButton::Right) {
        if let Some(block_type) = inventory.take_selected_block() {
            let pos = ray.block + ray.normal;

            // Locate chunk
            let (chunk_loc, inner_loc) = global_to_local_position(pos);

            // Try find chunk
            if let Some(chunk) = chunks.chunks.get_mut(&chunk_loc) {
                // Found chunk! Update block
                chunk.world[inner_loc.x][inner_loc.y][inner_loc.z] = block_type;

                // Rerender
                rerender_chunk_event.send(RerenderChunkFlag {
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
                chunks.create_chunk(chunk_loc, chunk, &mut commands, &assets, &mut meshes);

                rerender_chunk_event.send(RerenderChunkFlag {
                    chunk: chunk_loc,
                    context: RerenderChunkFlagContext::Surrounding,
                });
            }

            // Send network update
            send_packet.send(SendPacket(
                Protocol::BlockUpdate(BlockUpdate::new(block_type, pos.x, pos.y, pos.z)),
                UserId(0),
            ));
        }
    }
}
