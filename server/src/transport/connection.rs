use crate::events::authorize::AuthorizationEvent;
use crate::systems::authorization::GameUser;
use crate::TransportSystem;
use bevy::ecs::event::{EventReader, EventWriter};
use bevy::ecs::system::ResMut;
use rc_networking::constants::{EntityId};
use rc_networking::events::connection::NetworkConnectionEvent;


const MAX_PING_TIMEOUT_SECONDS: u64 = 10;
const PING_TIME_SECONDS: u64 = 15;

/// Accept connections by users and begin authorisation process
pub fn accept_connections(
    mut system: ResMut<TransportSystem>,
    mut connection_event_reader: EventReader<NetworkConnectionEvent>,
    mut authorization_writer: EventWriter<AuthorizationEvent>,
) {
    for connection_event in connection_event_reader.iter() {
        let user = GameUser {
            name: None,
            user_id: connection_event.client,
            entity_id: EntityId(connection_event.client.0),
            entity: None,
            loading: true,
        };

        system.clients.insert(connection_event.client, user);

        // Immediately authorize

        authorization_writer.send(AuthorizationEvent {
            user_id: connection_event.client,
        });
    }
}
