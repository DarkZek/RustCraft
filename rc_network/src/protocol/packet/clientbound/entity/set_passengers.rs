use crate::protocol::data::read_types::read_varint;
use crate::protocol::packet::clientbound::ClientBoundPacketType;
use std::io::Cursor;

#[derive(Debug)]
pub struct SetPassengersPacket {
    pub entity_id: i32,
    pub passengers: Vec<i32>,
}

impl ClientBoundPacketType for SetPassengersPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let entity_id = read_varint(buf);
        let passenger_len = read_varint(buf);
        let mut passengers = Vec::new();

        for _ in 0..passenger_len {
            passengers.push(read_varint(buf));
        }

        Box::new(SetPassengersPacket {
            entity_id,
            passengers,
        })
    }
}
