use bevy::log::warn;

use bevy::prelude::*;
use rc_protocol::constants::UserId;
use rc_protocol::protocol::serverbound::pong::Pong;
use rc_protocol::protocol::Protocol;

use crate::services::networking::TransportSystem;
use rc_protocol::types::{ReceivePacket, SendPacket};

pub fn connection_upkeep(
    system: ResMut<TransportSystem>,
    mut event_writer: EventWriter<ReceivePacket>,
) {
    // Check if we're connected to a server yet
    if system.socket.is_none() {
        return;
    }

    let client_disconnect = system.disconnect;

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

pub fn ping_reply(
    mut in_packets: EventReader<ReceivePacket>,
    mut out_packets: EventWriter<SendPacket>,
) {
    for packet in in_packets.iter() {
        if let Protocol::Ping(val) = packet.0 {
            out_packets.send(SendPacket(Protocol::Pong(Pong::from(val.code)), UserId(0)));
        }
    }
}
