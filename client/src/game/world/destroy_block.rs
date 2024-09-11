use crate::game::events::DestroyBlockEvent;
use crate::game::inventory::Inventory;
use crate::systems::chunk::builder::{RerenderChunkFlag, RerenderChunkFlagContext};
use crate::systems::chunk::ChunkSystem;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use nalgebra::Vector3;
use rc_shared::constants::UserId;
use rc_networking::protocol::Protocol;
use rc_networking::protocol::serverbound::destroy_block::DestroyBlock;
use rc_networking::types::SendPacket;
use rc_shared::block::BlockStates;
use rc_shared::chunk::ChunkSystemTrait;
use rc_shared::helpers::global_to_local_position;

enum DestroyBlockCommand {
    Skip,
    Replace(u32),
    Prevent,
}

// TODO: Move this into a more extensible system
fn get_destroy_block_providers(
) -> Vec<fn(block_id: u32, pos: Vector3<i32>, world: &mut World) -> DestroyBlockCommand> {
    vec![|block_id: u32, _pos: Vector3<i32>, world: &mut World| {
        let block_states = world.get_resource::<BlockStates>().unwrap();

        let ctable_id = block_states.get_by_id("mcv3::ConstructionTable").unwrap().0 as u32;
        let wood_id = block_states.get_by_id("mcv3::Wood").unwrap().0 as u32;
        let mut inventory = world.get_resource_mut::<Inventory>().unwrap();

        // If block is wood, and we're holding wood, make construction table
        if inventory.selected_block_id() == Some(wood_id) && block_id == wood_id {
            inventory.take_selected_block();
            return DestroyBlockCommand::Replace(ctable_id);
        }

        DestroyBlockCommand::Skip
    }]
}

pub fn destroy_block_system(
    world: &mut World,
    event_state: &mut SystemState<EventReader<DestroyBlockEvent>>,
) {
    let events = {
        event_state
            .get_mut(world)
            .read()
            .map(|event| event.clone())
            .collect::<Vec<DestroyBlockEvent>>()
    };

    for event in events {
        // Calculate what to do with the event
        let mut output_block_id = Some(0);

        // Run Destroy Block Providers
        for provider in get_destroy_block_providers() {
            match provider(event.block_id, event.position, world) {
                DestroyBlockCommand::Skip => {}
                DestroyBlockCommand::Replace(new_block_id) => {
                    output_block_id = Some(new_block_id);
                }
                DestroyBlockCommand::Prevent => output_block_id = None,
            }
        }

        let (chunk_loc, inner_loc) = global_to_local_position(event.position);

        // Apply block modification
        if let Some(new_block_id) = output_block_id {
            // Fetch chunk
            if let Some(chunk) = world
                .get_resource_mut::<ChunkSystem>()
                .unwrap()
                .get_raw_chunk_mut(&chunk_loc)
            {
                chunk.set(inner_loc, new_block_id);
            } else {
                warn!("Attempted to destroy block in unloaded chunk {:?}", event)
            }
        }

        // Rerender
        world.send_event(RerenderChunkFlag {
            chunk: chunk_loc,
            context: RerenderChunkFlagContext::Surrounding,
        });

        // Send network update
        world.send_event(SendPacket(
            Protocol::DestroyBlock(DestroyBlock::new(
                event.position.x,
                event.position.y,
                event.position.z,
            )),
            UserId(0),
        ));
    }
}
