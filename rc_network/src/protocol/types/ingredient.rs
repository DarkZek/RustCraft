use crate::protocol::types::slot::Slot;
use std::io::Cursor;
use crate::protocol::data::read_types::read_varint;

#[derive(Debug)]
pub struct Ingredient {
    count: i64,
    items: Vec<Slot>
}

impl Ingredient {
    pub fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Self {

        let count = read_varint(buf);
        let mut items = Vec::new();

        for _ in 0..count {
            items.push(Slot::deserialize(buf));
        }

        Ingredient {
            count,
            items
        }
    }
}