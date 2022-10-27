use std::fmt;
use crate::protocol::clientbound::chunk_update::CHUNK_UPDATE_BLOCKS_PER_PACKET;
use serde::{Serialize, Deserialize};

pub const CHUNK_SIZE: usize = 16;

pub type RawChunkData = [[[u32; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

#[derive(fmt::Debug, Hash, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct UserId(pub u64);

#[derive(fmt::Debug, Hash, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct EntityId(pub u64);