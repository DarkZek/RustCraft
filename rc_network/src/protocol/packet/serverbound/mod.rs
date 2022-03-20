pub mod player;

use crate::protocol::packet::serverbound::player::rotation::PlayerRotationPacket;
use crate::PacketBuilder;
use std::io::Cursor;

pub trait ServerBoundPacketType {
    fn serialize(&self) -> PacketBuilder;
}

#[derive(Debug)]
pub enum ServerBoundPacketData {
    PlayerRotation(PlayerRotationPacket),
}
