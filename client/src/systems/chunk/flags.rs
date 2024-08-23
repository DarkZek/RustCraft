use std::fmt::{Debug, Formatter};
use nalgebra::Vector3;
use crate::systems::chunk::ChunkSystem;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct ChunkFlags(pub u8);

pub enum ChunkFlagsBitMap {
    /// Is at the edge of the loaded terrain
    /// Chunks at the edge of the world aren't meshed as we don't have enough culling information
    AtEdge = 0b00000001,
    /// Chunk contains no blocks
    Empty = 0b00000010
}

impl Debug for ChunkFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ChunkFlags {{ AtEdge: {} }}", self.has_flag(ChunkFlagsBitMap::AtEdge))
    }
}

impl ChunkFlags {
    pub fn has_flag(&self, flag: ChunkFlagsBitMap) -> bool {
        let target: u8 = flag as u8;
        return (self.0 & target) == target;
    }

    pub fn add_flag(&mut self, flag: ChunkFlagsBitMap) {
        self.0 |= flag as u8;
    }

    pub fn remove_flag(&mut self, flag: ChunkFlagsBitMap) {
        self.0 &= !(flag as u8);
    }
}