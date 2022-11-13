use bevy::log::{info, warn};
use bevy::prelude::error;
use bevy::prelude::*;

use rc_protocol::error::ProtocolError;
use rc_protocol::protocol::serverbound::pong::Pong;
use rc_protocol::protocol::Protocol;
use std::io;
use rc_networking::client::ClientSocket;
use rc_protocol::constants::UserId;
use rc_protocol::types::{ReceivePacket, SendPacket};
use crate::services::networking::TransportSystem;

pub fn connection_upkeep(
    mut system: ResMut<TransportSystem>,
    mut event_writer: EventWriter<ReceivePacket>,
) {
    // Check if we're connected to a server yet
    if system.socket.is_none() {
        return;
    }

    let mut client_disconnect = system.disconnect;

    let poll = system.socket.as_ref().unwrap().poll();

    event_writer.send_batch(poll.packets.into_iter());

    if client_disconnect {
        warn!("Disconnected from server: Unexpected Disconnection");
        //TODO: Add
    }
}

pub fn send_packets(mut system: ResMut<TransportSystem>, mut packets: EventReader<SendPacket>) {
    if let Some(socket) = &mut system.socket {
        for packet in packets.iter() {
            socket.send_packet(packet.clone());
        }
    }
}
