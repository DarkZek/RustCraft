use std::sync::atomic::Ordering;

use crate::game::transform::Transform;
use crate::game::update::BlockUpdateEvent;
use crate::game::world::data::GAME_OBJECT_ID_COUNTER;
use crate::helpers::global_to_local_position;
use crate::systems::game_object::spawn::SpawnGameObjectRequest;
use crate::{TransportSystem, WorldData};
use bevy::ecs::prelude::*;
use bevy::log::info;
use nalgebra::{Quaternion, Vector3, Vector4};
use rand::Rng;
use rc_networking::protocol::clientbound::block_update::BlockUpdate;
use rc_networking::protocol::clientbound::entity_moved::EntityMoved;
use rc_networking::protocol::clientbound::entity_rotated::EntityRotated;
use rc_networking::protocol::Protocol;
use rc_networking::types::{ReceivePacket, SendPacket};
use rc_shared::block::BlockStates;
use rc_shared::constants::GameObjectId;
use rc_shared::game_objects::{GameObjectData, ItemDropGameObjectData};
use rc_shared::item::types::ItemStack;
use rc_shared::item::ItemStates;
use rc_shared::viewable_direction::BLOCK_SIDES;
use bevy::log::warn;
use crate::game::entity::{DirtyPosition, DirtyRotation};

pub fn receive_message_event(
    mut event_reader: EventReader<ReceivePacket>,
    mut event_writer: EventWriter<SendPacket>,
    mut global: ResMut<WorldData>,
    system: Res<TransportSystem>,
    mut transforms: Query<&mut Transform>,
    mut block_update_writer: EventWriter<BlockUpdateEvent>,
    mut ew: EventWriter<SpawnGameObjectRequest>,
    block_states: Res<BlockStates>,
    item_states: Res<ItemStates>,
    mut commands: Commands
) {
    for event in event_reader.read() {
        match &event.0 {
            Protocol::PlayerMove(packet) => {
                // Update all other clients
                let Some(entity) = system.clients.get(&event.1).unwrap().game_object_id else {
                    continue
                };

                if let Some(val) = global.get_game_object(&entity) {
                    // Move player in ecs
                    transforms.get_mut(val.clone()).unwrap().position =
                        Vector3::new(packet.x, packet.y, packet.z);
                    commands.entity(val.clone()).insert(DirtyPosition);
                    commands.entity(val).insert(DirtyRotation);
                } else {
                    warn!("Player {:?} tried to move that wasn't spawned in", event.1);
                }
            }
            Protocol::PlayerRotate(packet) => {
                // Update all other clients
                let Some(entity) = system.clients.get(&event.1).unwrap().game_object_id else {
                    continue
                };

                if let Some(val) = global.get_game_object(&entity) {
                    // Move player in ecs
                    transforms.get_mut(val).unwrap().rotation =
                        Quaternion::from_vector(Vector4::new(packet.x, packet.y, packet.z, packet.w));
                    commands.entity(val.clone()).insert(DirtyPosition);
                    commands.entity(val).insert(DirtyRotation);
                } else {
                    warn!("Player {:?} tried to move that wasn't spawned in", event.1);
                }
            }
            Protocol::BlockUpdate(packet) => {
                // TODO: Don't trust user input
                let packet = BlockUpdate::new(packet.id, packet.x, packet.y, packet.z);

                for (client, _) in &system.clients {
                    if *client == event.1 {
                        continue;
                    }
                    info!("Block update packet sent to {:?}", client);
                    event_writer.send(SendPacket(Protocol::BlockUpdate(packet), *client));
                }

                let (chunk_loc, inner_loc) =
                    global_to_local_position(Vector3::new(packet.x, packet.y, packet.z));

                // Store
                let old_block_id = if let Some(chunk) = global.chunks.get_mut(&chunk_loc) {
                    let block_id = chunk.world.get(inner_loc);
                    // Found chunk! Update block
                    chunk.world.set(inner_loc, packet.id);
                    block_id
                } else {
                    warn!("{:?} attempted to break block in unloaded chunk. Skipping {:?}", event.1, chunk_loc);
                    continue;
                };

                // Trigger block update for all surrounding blocks
                block_update_writer.send(BlockUpdateEvent {
                    pos: Vector3::new(packet.x, packet.y, packet.z),
                });
                for side in &BLOCK_SIDES {
                    block_update_writer.send(BlockUpdateEvent {
                        pos: Vector3::new(packet.x, packet.y, packet.z) + side,
                    });
                }

                // Spawn block drops after destroying
                if packet.id == 0 {
                    let drops = calculate_drops(&block_states, &item_states, old_block_id);

                    for drop in drops {
                        info!("Spawning item drop with item {:?}", drop);
                        ew.send(SpawnGameObjectRequest {
                            transform: Transform::from_translation(Vector3::new(
                                packet.x as f32 + 0.5,
                                packet.y as f32 + 0.5,
                                packet.z as f32 + 0.5,
                            )),
                            data: GameObjectData::ItemDrop(ItemDropGameObjectData {
                                item_stack: drop,
                            }),
                            id: GameObjectId(GAME_OBJECT_ID_COUNTER.fetch_add(1, Ordering::SeqCst)),
                            entity: None,
                        });
                    }
                }
            }
            _ => {}
        }
    }
}

fn calculate_drops(
    block_states: &BlockStates,
    item_states: &ItemStates,
    block_id: u32,
) -> Vec<ItemStack> {
    let mut drop = Vec::new();

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
                drop.push(ItemStack::new(item.clone(), amount));
            }
        }
    }

    drop
}