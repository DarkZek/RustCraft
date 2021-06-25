use crate::protocol::data::read_types::{
    read_bool, read_float, read_position, read_string, read_unsignedbyte, read_varint,
};
use crate::protocol::types::slot::Slot;
use crate::protocol::types::PVarType;
use std::io::Cursor;

#[derive(Debug)]
pub struct EntityMetadata {
    index: u8,
    ty: Option<i64>,
    value: Option<Box<PVarType>>,
}
// TODO: Fix problems deserializing
impl EntityMetadata {
    pub fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Self {
        let index = read_unsignedbyte(buf);

        if index == 0xff {
            return EntityMetadata {
                index,
                ty: None,
                value: None,
            };
        }

        let ty = read_varint(buf);

        // https://wiki.vg/Entities#Entity_Metadata_Format
        let value = match ty {
            0 => PVarType::UnsignedByte(read_unsignedbyte(buf)),
            1 => PVarType::VarInt(read_varint(buf)),
            2 => PVarType::Float(read_float(buf)),
            3 => PVarType::String(read_string(buf)),
            4 => PVarType::String(read_string(buf)),
            5 => {
                let has_chat = read_bool(buf);
                PVarType::OptChat(if has_chat {
                    Some(read_string(buf))
                } else {
                    None
                })
            }
            6 => PVarType::Slot(Slot::deserialize(buf)),
            7 => PVarType::Boolean(read_bool(buf)),
            8 => PVarType::Rotation([read_float(buf), read_float(buf), read_float(buf)]),
            9 => PVarType::Position(read_position(buf)),
            13 => PVarType::VarInt(read_varint(buf)),
            18 => PVarType::VarInt(read_varint(buf)),
            _ => panic!("Bruh you didn't implement the type!: {}", ty),
        };

        EntityMetadata {
            index,
            ty: Some(ty),
            value: Some(Box::new(value)),
        }
    }
}
