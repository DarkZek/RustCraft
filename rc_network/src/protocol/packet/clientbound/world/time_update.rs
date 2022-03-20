use crate::protocol::data::read_types::read_long;
use crate::protocol::packet::clientbound::ClientBoundPacketType;
use std::io::Cursor;

#[derive(Debug)]
pub struct TimeUpdatePacket {
    pub world_age: i64,
    pub time_of_day: i64,
}

impl ClientBoundPacketType for TimeUpdatePacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let world_age = read_long(buf);
        let time_of_day = read_long(buf);

        Box::new(TimeUpdatePacket {
            world_age,
            time_of_day,
        })
    }
}
