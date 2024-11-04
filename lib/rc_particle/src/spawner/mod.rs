pub mod spawn;
pub mod despawn;
pub mod detect;

use std::time::Duration;
use bevy::prelude::{Component, Handle, Mesh};
use rc_shared::aabb::Aabb;
use rc_shared::atlas::TextureAtlasIndex;

// TODO: Make transform a required component
#[derive(Component)]
pub struct ParticleSpawner {
    /// The base position
    pub area: SpawnArea,
    /// How many are spawned per second
    pub spawn_rate: f32,
    /// Texture
    pub texture: TextureAtlasIndex,
    /// Time to live
    pub ttl: Duration,
}

#[derive(Component)]
pub(crate) struct ParticleSpawnerMeta {
    pub simulated_to: u128,
    pub mesh: Handle<Mesh>
}

pub enum SpawnArea {
    Point,
    Area(Aabb)
}