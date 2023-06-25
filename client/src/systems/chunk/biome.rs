#[derive(Eq, PartialEq, Debug)]
pub struct ChunkEnvironment {
    pub climate: Climate,
    pub terrain: Terrain,
    pub vegetation: Vegetation,
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
