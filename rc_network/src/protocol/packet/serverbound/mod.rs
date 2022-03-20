pub mod player;

use crate::protocol::packet::serverbound::player::position::PlayerPositionPacket;
use crate::protocol::packet::serverbound::player::rotation::PlayerRotationPacket;
use crate::PacketBuilder;
use std::io::Cursor;

pub trait ServerBoundPacketType {
    fn serialize(&self) -> PacketBuilder;
}

#[derive(Debug)]
pub enum ServerBoundPacketData {
    PlayerRotation(PlayerRotationPacket),
    PlayerPosition(PlayerPositionPacket),
}

impl ServerBoundPacketData {
    pub fn serialize(&self) -> PacketBuilder {
        match self {
            ServerBoundPacketData::PlayerRotation(packet) => packet.serialize(),
            ServerBoundPacketData::PlayerPosition(packet) => packet.serialize(),
        }
    }
}
