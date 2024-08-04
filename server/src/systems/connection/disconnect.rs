use std::fs;
use crate::{TransportSystem, WorldData};
use bevy::ecs::event::EventReader;
use bevy::ecs::prelude::{Commands, EventWriter};
use bevy::ecs::system::ResMut;
use bevy::prelude::{Query, Res, warn};
use rc_networking::events::disconnect::NetworkDisconnectionEvent;
use rc_networking::protocol::clientbound::despawn_game_object::DespawnGameObject;
use crate::game::world::deserialized_player::DeserializedPlayerData;
use crate::game::transform::Transform;

use rc_networking::protocol::Protocol;
use rc_networking::types::SendPacket;
use rc_shared::helpers::global_f32_to_local_position;
use crate::config::ServerConfig;
use crate::game::inventory::Inventory;

pub fn disconnection_event(
    mut event_reader: EventReader<NetworkDisconnectionEvent>,
    mut commands: Commands,
    mut world: ResMut<WorldData>,
    mut writer: EventWriter<SendPacket>,
    mut clients: ResMut<TransportSystem>,
    config: Res<ServerConfig>,
    query: Query<(&Transform, &Inventory)>,
) {
    for event in event_reader.read() {
        let entry = clients.clients.remove(&event.client).unwrap();

        let Some(game_object_id) = &entry.game_object_id else {
            warn!("Tried to disconnect already despawned player {:?}", entry.user_id);
            continue
        };

        let (transform, inventory) = query
            .get(world.get_game_object(&game_object_id).unwrap())
            .unwrap();

        let (chunk_pos, _) = global_f32_to_local_position(transform.position);

        if let Some(eid) = world.remove_game_object(game_object_id, chunk_pos) {

            if config.save_world {

                // TODO: Move to more centralized place
                // Save player data
                let data = DeserializedPlayerData {
                    position: transform.position,
                    rotation: transform.rotation,
                    inventory: inventory.clone()
                };

                fs::write(
                    format!("./world/players/{}", entry.user_id.0),
                    serde_json::to_string(&data).unwrap(),
                )
                .unwrap();
            }

            // Delete game_object
            commands.entity(eid).despawn();

            // Send all other players a disconnection event
            for (uid, _) in &clients.clients {
                writer.send(SendPacket(
                    Protocol::DespawnGameObject(DespawnGameObject::new(*game_object_id)),
                    *uid,
                ));
            }
        }
    }
}
