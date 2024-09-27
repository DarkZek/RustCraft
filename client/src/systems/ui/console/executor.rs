use bevy::prelude::{EventWriter, ResMut};
use crate::systems::ui::console::ConsoleData;
use rc_networking::protocol::Protocol;
use rc_networking::protocol::serverbound::player_chat::PlayerChat;
use rc_networking::types::SendPacket;
use rc_shared::constants::UserId;

pub enum CommandExecuted {
    Message(String)
}

pub fn execute_commands(
    mut data: ResMut<ConsoleData>,
    mut send_packet: EventWriter<SendPacket>
) {
    for message in data.messages_sent.drain(..) {
        let packet = Protocol::PlayerChat(PlayerChat {
            message
        });
        send_packet.send(SendPacket(packet, UserId(0)));
    }
}