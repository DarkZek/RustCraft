use crate::events::connection::ConnectionEvent;
use crate::events::disconnect::DisconnectionEvent;
use crate::systems::authorization::GameUser;
use crate::TransportSystem;
use bevy::ecs::event::{EventReader, EventWriter};
use bevy::ecs::system::ResMut;
use rc_client::rc_networking::renet::ServerEvent;
use rc_client::rc_networking::constants::{EntityId, UserId};

const MAX_PING_TIMEOUT_SECONDS: u64 = 10;
const PING_TIME_SECONDS: u64 = 15;

/// Accept connections by users and begin authorisation process
pub fn accept_connections(
    mut system: ResMut<TransportSystem>,
    mut server_events: EventReader<ServerEvent>,
    mut connection_event_writer: EventWriter<ConnectionEvent>,
    mut disconnect_event_writer: EventWriter<DisconnectionEvent>,
) {
    server_events
        .iter()
        .for_each(|v: &ServerEvent| {
            match v {
                ServerEvent::ClientConnected(id, _) => {
                    let user_id = UserId(*id);
                    let user = GameUser {
                        name: None,
                        user_id,
                        entity_id: EntityId(*id),
                    };

                    system.clients.insert(user_id, user);

                    connection_event_writer.send(ConnectionEvent::new(user_id));
                }
                ServerEvent::ClientDisconnected(id) => {
                    let user_id = UserId(*id);
                    if let Some(user) = system.clients.remove(&user_id) {
                        disconnect_event_writer.send(DisconnectionEvent { client: user_id, user });
                    };
                }
            }
        });
}
