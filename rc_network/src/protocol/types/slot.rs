use std::io::Cursor;
use crate::protocol::data::read_types::{read_bool, read_varint, read_unsignedbyte};
use byteorder::ReadBytesExt;
use nbt::Blob;

#[derive(Debug)]
pub struct Slot {
    present: bool,
    item_id: Option<i64>,
    item_count: Option<u8>,
    nbt: Option<Blob>
}

impl Slot {
    pub fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Self {

        let present = read_bool(buf);

        if !present {
            return Slot {
                present,
                item_id: None,
                item_count: None,
                nbt: None
            };
        }

        let item_id = Some(read_varint(buf));
        let item_count = Some(read_unsignedbyte(buf));

        let nbt_start = buf.read_u8().unwrap();

        let nbt = if nbt_start != 0 {
            buf.set_position(buf.position() - 1);
            Some(Blob::from_reader(buf).unwrap())
        } else {
            None
        };

        Slot {
            present,
            item_id,
            item_count,
            nbt
        }
    }
}