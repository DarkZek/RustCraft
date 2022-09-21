use crate::protocol::clientbound::chunk_update::CHUNK_UPDATE_BLOCKS_PER_PACKET;

pub const CHUNK_SIZE: usize = 16;

pub type RawChunkData = [[[u32; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

pub type PartialChunkData = [u8; CHUNK_UPDATE_BLOCKS_PER_PACKET];
