use crate::game::events::DestroyBlockEvent;
use crate::game::inventory::Inventory;
use crate::systems::chunk::builder::{RerenderChunkFlag, RerenderChunkFlagContext};
use crate::systems::chunk::ChunkSystem;
use bevy::ecs::event::ManualEventReader;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use nalgebra::Vector3;
use rand::Rng;
use rc_shared::constants::UserId;
use rc_networking::protocol::clientbound::block_update::BlockUpdate;
use rc_networking::protocol::Protocol;
use rc_networking::types::SendPacket;
use rc_shared::block::BlockStates;
use rc_shared::chunk::ChunkSystemTrait;
use rc_shared::helpers::global_to_local_position;
use rc_shared::item::types::ItemStack;
use rc_shared::item::ItemStates;

enum DestroyBlockCommand {
    Skip,
    Replace(u32),
    Prevent,
}

// TODO: Move this into a more extensible system
fn get_destroy_block_providers(
) -> Vec<fn(block_id: u32, pos: Vector3<i32>, world: &mut World) -> DestroyBlockCommand> {
    vec![|block_id: u32, pos: Vector3<i32>, world: &mut World| {
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
    mut world: &mut World,
    mut event_state: &mut SystemState<EventReader<DestroyBlockEvent>>,
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
                chunk[inner_loc.x][inner_loc.y][inner_loc.z] = new_block_id;
            } else {
                warn!("Attempted to destroy block in unloaded chunk {:?}", event)
            }
        }

        info!("Destroyed {:?}", event.position);

        // Rerender
        world.send_event(RerenderChunkFlag {
            chunk: chunk_loc,
            context: RerenderChunkFlagContext::Surrounding,
        });

        // Send network update
        world.send_event(SendPacket(
            Protocol::BlockUpdate(BlockUpdate::new(
                0,
                event.position.x,
                event.position.y,
                event.position.z,
            )),
            UserId(0),
        ))
    }
}

fn apply_drops(
    block_states: &BlockStates,
    item_states: &ItemStates,
    block_id: u32,
    inventory: &mut Inventory,
) {
    for drops in block_states.loot_tables.get(block_id as usize).unwrap() {
        if let Some(item) = item_states.states.get(drops.item_id) {
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
}
