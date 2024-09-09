use bevy::{ecs::{event::EventWriter, system::{Commands, Query}}};
use rc_networking::{types::SendPacket};
use rc_shared::game_objects::{ItemDropGameObjectData, PlayerGameObjectData};
use crate::game::{game_object::GameObject, inventory::Inventory, transform::Transform};
use bevy::ecs::entity::Entity;
use crate::systems::game_object::despawn::DespawnGameObject;

const ITEM_COLLECTION_RADIUS: f32 = 2.0;

pub fn collect_items(
    players_query: Query<(Entity, &PlayerGameObjectData, &Transform)>,
    mut items_query: Query<(Entity, &GameObject, &mut ItemDropGameObjectData, &Transform)>,
    mut inventory_query: Query<&mut Inventory>,
    mut command: Commands,
    send_packet: EventWriter<SendPacket>,
) {
    for (entity, game_object, mut item_drop, transform) in items_query.iter_mut() {

        if item_drop.item_stack.amount == 0 {
            // Already collected
            continue
        }

        // Find all nearby items
        for (player_entity, player_data, player_pos) in players_query.iter() {
            // Get distance between item and player
            let dist = (player_pos.position - transform.position).magnitude();

            if dist < ITEM_COLLECTION_RADIUS {
                // Add to user inventory
                let mut inventory = inventory_query.get_mut(player_entity).unwrap();
                inventory.push_item(item_drop.item_stack.clone());

                // Prevent it from being collected twice
                item_drop.item_stack.amount = 0;

                command.entity(entity).insert(DespawnGameObject);

                break;
            }
        }
    }
}