use crate::protocol::types::PVarType;

pub mod read_types;
pub mod write_types;
pub mod writer;
pub mod reader;

pub struct Packet {
    pub id: u16,
    pub len: u32,
    pub tokens: Vec<PVarType>,
}
