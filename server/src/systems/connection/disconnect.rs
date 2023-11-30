use crate::{TransportSystem, WorldData};
use bevy::ecs::event::EventReader;
use bevy::ecs::prelude::{Commands, EventWriter};
use bevy::ecs::system::ResMut;
use bevy::prelude::Query;
use rc_networking::events::disconnect::NetworkDisconnectionEvent;
use rc_networking::protocol::clientbound::despawn_game_object::DespawnGameObject;

use crate::game::transform::Transform;
use crate::helpers::global_to_local_position;
use rc_networking::protocol::Protocol;
use rc_networking::types::SendPacket;
use rc_shared::helpers::global_f32_to_local_position;

pub fn disconnection_event(
    mut event_reader: EventReader<NetworkDisconnectionEvent>,
    mut commands: Commands,
    mut world: ResMut<WorldData>,
    mut writer: EventWriter<SendPacket>,
    mut clients: ResMut<TransportSystem>,
    query: Query<&Transform>,
) {
    for event in event_reader.read() {
        let entry = clients.clients.remove(&event.client).unwrap();

        let transform = query
            .get(world.get_game_object(&entry.game_object_id).unwrap())
            .unwrap();

        let (chunk_pos, _) = global_f32_to_local_position(transform.position);

        if let Some(eid) = world.remove_game_object(&entry.game_object_id, chunk_pos) {
            // Delete game_object
            commands.entity(eid).despawn();

            // Send all other players a disconnection event
            for (uid, _) in &clients.clients {
                writer.send(SendPacket(
                    Protocol::DespawnGameObject(DespawnGameObject::new(entry.game_object_id)),
                    *uid,
                ));
            }
        }
    }
}
