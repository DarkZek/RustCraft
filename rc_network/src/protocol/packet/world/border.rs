use crate::protocol::data::read_types::{read_double, read_varint, read_varlong};
use crate::protocol::packet::PacketType;
use std::io::Cursor;

#[derive(Debug)]
pub struct WorldBorderPacket {
    pub data: WorldBorderData,
}

impl PacketType for WorldBorderPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let action = read_varint(buf);

        let data = match action {
            0 => WorldBorderData::SetSize(read_double(buf)),
            1 => WorldBorderData::LerpSize(read_double(buf), read_double(buf), read_varint(buf)),
            2 => WorldBorderData::SetCenter(read_double(buf), read_double(buf)),
            3 => WorldBorderData::Initialize(WorldBorderDataInitialize {
                x: read_double(buf),
                z: read_double(buf),
                old_diameter: read_double(buf),
                new_diameter: read_double(buf),
                speed: read_varlong(buf),
                portal_teleport_boundary: read_varint(buf),
                warning_time: read_varint(buf),
                warning_blocks: read_varint(buf),
            }),
            4 => WorldBorderData::SetWarningTime(read_varint(buf)),
            5 => WorldBorderData::SetWarningBlocks(read_varint(buf)),
            _ => panic!(),
        };

        Box::new(WorldBorderPacket { data })
    }
}

#[derive(Debug)]
pub enum WorldBorderData {
    SetSize(f64),
    LerpSize(f64, f64, i64),
    SetCenter(f64, f64),
    Initialize(WorldBorderDataInitialize),
    SetWarningTime(i64),
    SetWarningBlocks(i64),
}

#[derive(Debug)]
pub struct WorldBorderDataInitialize {
    x: f64,
    z: f64,
    old_diameter: f64,
    new_diameter: f64,
    speed: i64,
    portal_teleport_boundary: i64,
    warning_time: i64,
    warning_blocks: i64,
}
