pub mod highlight;

use crate::helpers::{from_bevy_vec3, global_to_local_position};
use crate::systems::asset::AssetService;
use crate::systems::chunk::ChunkSystem;
use crate::systems::physics::raycasts::do_raycast;
use bevy::prelude::*;
use rand::Rng;

use crate::game::blocks::states::BlockStates;
use crate::game::inventory::Inventory;
use crate::game::item::states::ItemStates;
use crate::game::item::ItemStack;
use crate::systems::chunk::builder::{RerenderChunkFlag, RerenderChunkFlagContext};
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
    mut inventory: ResMut<Inventory>,
    blocks: Res<BlockStates>,
    mut rerender_chunks: EventWriter<RerenderChunkFlag>,
    mut meshes: ResMut<Assets<Mesh>>,
    items: Res<ItemStates>,
    block_states: Res<BlockStates>,
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
    if let Some(chunk) = chunks.chunks.get_mut(&chunk_loc) {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            let block_id = chunk.world[inner_loc.x][inner_loc.y][inner_loc.z];

            for drops in block_states.loot_tables.get(block_id as usize).unwrap() {
                if let Some(item) = items.states.get(drops.item_id) {
                    let mut amount = drops.chance.floor() as u32;

                    // Partial chance means partial chance to get the drop
                    if drops.chance % 1.0 > 0.0
                        && rand::thread_rng().gen_range(0.0..=1.0) <= drops.chance % 1.0
                    {
                        amount += 1;
                    }

                    if amount > 0 {
                        inventory.push_item(ItemStack::new(item.clone(), amount));
                        info!("Added {} {} to inventory", amount, item.name);
                    }
                }
            }

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
        if let Some(block_type) = inventory.take_selected_block_id() {
            let pos = ray.block + ray.normal;

            // Locate chunk
            let (chunk_loc, inner_loc) = global_to_local_position(pos);

            // Try find chunk
            if let Some(chunk) = chunks.chunks.get_mut(&chunk_loc) {
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
