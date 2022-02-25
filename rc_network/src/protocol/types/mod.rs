use crate::protocol::types::slot::Slot;
use nbt::Blob;

pub mod chunk;
pub mod entity_metadata;
pub mod ingredient;
pub mod modifier_data;
pub mod slot;

#[derive(Debug)]
pub enum PVarType {
    UnsignedByte(u8),
    VarInt(i32),
    String(String),
    // For byte arrays preceded by var ints determining the length
    VarIntByteArray(Vec<u8>),
    // For byte arrays that use the remaining length
    ByteArray(Vec<u8>),
    Int(i32),
    Long(i64),
    Boolean(bool),
    Float(f32),
    NBT(Blob),
    IntArray(Vec<i32>),
    NBTArray(Vec<Blob>),
    OptChat(Option<String>),
    Slot(Slot),
    Rotation([f32; 3]),
    Position([i64; 3]),
}

pub enum PVarTypeTemplate {
    UnsignedByte,
    VarInt,
    String,
    // For byte arrays preceded by var ints determining the length
    VarIntByteArray,
    // For byte arrays that use the remaining length
    ByteArray,
    Int,
    Long,
    Boolean,
    Float,
    NBT,
    IntArray(u32),
    NBTArray,
    ChunkData(usize),
}
