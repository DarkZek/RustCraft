#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ChunkEnvironment {
    FOREST,
    PLAIN
}

pub type EnvironmentMap = [[[EnvironmentEntry; 16]; 16]; 16];

#[derive(PartialEq, Debug, Copy, Clone)]
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
