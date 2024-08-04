use std::time::Instant;
use crate::systems::asset::AssetService;
use crate::systems::chunk::ChunkSystem;
use crate::systems::physics::raycasts::do_raycast;
use bevy::prelude::*;
use nalgebra::Vector3;
use crate::game::inventory::Inventory;
use crate::systems::chunk::builder::{RerenderChunkFlag, RerenderChunkFlagContext};
use rc_shared::constants::UserId;
use rc_networking::protocol::clientbound::block_update::BlockUpdate;
use rc_networking::protocol::Protocol;
use rc_networking::types::SendPacket;
use rc_shared::block::BlockStates;
use rc_shared::helpers::{from_bevy_vec3, global_to_local_position};
use rc_shared::CHUNK_SIZE;
use crate::systems::camera::freecam::Freecam;
use crate::systems::camera::MainCamera;

pub struct MouseInteractionLocals {
    left_clicking_started: Option<Instant>,
    left_clicking_target: Vector3<i32>
}

pub fn mouse_interaction_place(
    freecam: Res<Freecam>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    camera: Query<&Transform, With<MainCamera>>,
    assets: Res<AssetService>,
    mut rerender_chunk_event: EventWriter<RerenderChunkFlag>,
    mut chunks: ResMut<ChunkSystem>,
    blocks: Res<BlockStates>,
    mut inventory: ResMut<Inventory>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut send_packet: EventWriter<SendPacket>,
) {

    if freecam.enabled || !mouse_button_input.just_pressed(MouseButton::Right) {
        return
    }

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

    if let Some(block_type) = inventory.selected_block_id() {
        let pos = ray.block + ray.normal;

        // Locate chunk
        let (chunk_loc, inner_loc) = global_to_local_position(pos);

        // Try find chunk
        if let Some(chunk) = chunks.chunks.get_mut(&chunk_loc) {

            if chunk.world[inner_loc.x][inner_loc.y][inner_loc.z] != 0 {
                // Trying to place a block on another block
                return
            }

            // Found chunk! Update block
            chunk.world[inner_loc.x][inner_loc.y][inner_loc.z] = block_type;

            // Rerender
            rerender_chunk_event.send(RerenderChunkFlag {
                chunk: chunk_loc,
                context: RerenderChunkFlagContext::Surrounding,
            });

            debug!(
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

        inventory.take_selected_block();

        // Send network update
        send_packet.send(SendPacket(
            Protocol::BlockUpdate(BlockUpdate::new(block_type, pos.x, pos.y, pos.z)),
            UserId(0),
        ));
    }
}
