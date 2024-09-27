use bevy::prelude::{EventReader, EventWriter, info, Res};
use rc_networking::protocol::clientbound::chat::ChatSent;
use rc_networking::protocol::Protocol;
use rc_networking::types::{ReceivePacket, SendPacket};
use crate::game::commands::ExecuteCommandRequest;
use crate::transport::TransportSystem;

// Broadcasts chat messages from clients to all other clients
pub fn broadcast_chat(
    mut receive_packet: EventReader<ReceivePacket>,
    mut send_packet: EventWriter<SendPacket>,
    mut execute_command: EventWriter<ExecuteCommandRequest>,
    transport_system: Res<TransportSystem>
) {
    for packet in receive_packet.read() {
        let Protocol::PlayerChat(player_chat) = &packet.0 else {
            continue;
        };

        if player_chat.message.starts_with("/") {
            execute_command.send(ExecuteCommandRequest {
                user_id: packet.1,
                message: player_chat.message[1..].to_string()
            });
            continue
        }

        let user = transport_system.clients.get(&packet.1).unwrap();

        let message = format!("{}: {}", user.name, player_chat.message);

        info!("[Chat] {}", message);

        let packet = Protocol::ChatSent(
            ChatSent {
                message
            }
        );

        for client in transport_system.clients.keys() {
            send_packet.send(SendPacket(packet.clone(), *client));
        }
    }
}