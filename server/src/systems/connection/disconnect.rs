use crate::{TransportSystem, WorldData};
use bevy::ecs::event::EventReader;
use bevy::ecs::prelude::{Commands, EventWriter};
use bevy::ecs::system::ResMut;
use rc_networking::events::disconnect::NetworkDisconnectionEvent;
use rc_networking::protocol::clientbound::despawn_game_object::DespawnGameObject;

use rc_networking::protocol::Protocol;
use rc_networking::types::SendPacket;

pub fn disconnection_event(
    mut event_reader: EventReader<NetworkDisconnectionEvent>,
    mut commands: Commands,
    mut world: ResMut<WorldData>,
    mut writer: EventWriter<SendPacket>,
    mut clients: ResMut<TransportSystem>,
) {
    for event in event_reader.read() {
        let entry = clients.clients.remove(&event.client).unwrap();

        if let Some(eid) = world.entities.remove(&entry.entity_id) {
            // Delete game_object
            commands.entity(eid).despawn();

            // Send all other players a disconnection event
            for (uid, _) in &clients.clients {
                writer.send(SendPacket(
                    Protocol::DespawnGameObject(DespawnGameObject::new(entry.entity_id)),
                    *uid,
                ));
            }
        }
    }
}
