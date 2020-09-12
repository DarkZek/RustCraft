use std::io::Cursor;
use crate::protocol::data::read_types::{read_unsignedbyte, read_uuid, read_double};

// https://wiki.vg/Protocol#Entity_Properties

#[derive(Debug)]
pub struct ModifierData {
    uuid: u128,
    amount: f64,
    operation: u8
}

impl ModifierData {
    pub fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Self {

        let uuid = read_uuid(buf);
        let amount = read_double(buf);
        let operation = read_unsignedbyte(buf);

        ModifierData {
            uuid,
            amount,
            operation
        }
    }
}