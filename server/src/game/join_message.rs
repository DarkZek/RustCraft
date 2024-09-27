use bevy::prelude::{EventReader, EventWriter, info, Res};
use rc_networking::events::disconnect::NetworkDisconnectionEvent;
use rc_networking::protocol::clientbound::chat::ChatSent;
use rc_networking::protocol::Protocol;
use rc_networking::types::SendPacket;
use crate::events::join::PlayerSpawnEvent;
use crate::transport::TransportSystem;

pub fn join_message(
    mut events: EventReader<PlayerSpawnEvent>,
    users: Res<TransportSystem>,
    mut packets: EventWriter<SendPacket>
) {
    for join_event in events.read() {

        let Some(game_user) = users.clients.get(&join_event.id) else {
            continue
        };

        let username = game_user.name.clone();

        for (user_id, user) in &users.clients {
            packets.send(SendPacket(Protocol::ChatSent(ChatSent {
                message: format!("{} joined the game", username)
            }), *user_id));
        }

        info!("[Chat] {} joined the game", username);

        let connected_users = users.clients.iter()
            .map(|(_, v)| v.name.clone())
            .reduce(|a, b| format!("{}, {}", a, b))
            .unwrap();

        packets.send(SendPacket(Protocol::ChatSent(ChatSent {
            message: format!("Connected Users: [{}]", connected_users)
        }), join_event.id));
    }
}

pub fn leave_message(
    mut events: EventReader<NetworkDisconnectionEvent>,
    users: Res<TransportSystem>,
    mut packets: EventWriter<SendPacket>
) {
    for leave_event in events.read() {

        let Some(game_user) = users.clients.get(&leave_event.client) else {
            continue
        };

        let username = game_user.name.clone();

        for (user_id, _) in &users.clients {
            packets.send(SendPacket(Protocol::ChatSent(ChatSent {
                message: format!("{} left the game", username)
            }), *user_id));
        }

        info!("[Chat] {} left the game", username);
    }
}