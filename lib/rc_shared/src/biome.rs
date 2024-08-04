use crate::relative_chunk_flat_map::RelativeChunkFlatMap;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ChunkEnvironment {
    FOREST,
    PLAIN
}

pub type EnvironmentMap = RelativeChunkFlatMap<EnvironmentEntry>;

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub struct EnvironmentEntry {
    pub climate: f64,
    pub terrain: f64,
    pub vegetation: f64,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Climate {
    Tropic,    // Warm
    Temperate, // Middle
    Frigid,    // Cold
}

#[derive(Eq, PartialEq, Debug)]
pub enum Terrain {
    Hills,
    Plain,
    Forest,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Vegetation {
    None,
    Grass,
    Trees,
}
