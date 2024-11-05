pub mod spawn;
pub mod despawn;
pub mod detect;
pub mod simulation;
pub mod expire;

use std::time::Duration;
use bevy::prelude::{Component, Handle, Mesh};
use nalgebra::Vector3;
use rand::Rng;
use rc_shared::aabb::Aabb;
use rc_shared::atlas::TextureAtlasIndex;
use crate::spawner::simulation::ParticleSimulationSettings;

// TODO: Make transform a required component
#[derive(Component)]
pub struct ParticleSpawner {
    /// The base position
    pub area: SpawnArea,
    /// How many are spawned per second
    pub spawn_rate: f32,
    /// Texture
    pub texture: TextureAtlasIndex,
    /// Time to live of each particle
    pub particle_ttl: Duration,
    /// Time to live of spawner
    pub expires: Duration,
    // Simulation settings
    pub simulation: Option<ParticleSimulationSettings>
}

#[derive(Component)]
pub(crate) struct ParticleSpawnerMeta {
    pub i: usize,
    pub simulated_to: u128,
    pub spawned_at: f32,
    pub mesh: Handle<Mesh>
}

pub enum SpawnArea {
    Point,
    Area(Aabb),
    Custom(Box<dyn (Fn(usize) -> Vector3<f32>) + Send + Sync>),
}

impl SpawnArea {
    pub fn get_offset(&self, i: usize) -> Vector3<f32> {
        let mut rng = rand::thread_rng();
        match self {
            SpawnArea::Point => Vector3::new(0.0, 0.0, 0.0),
            SpawnArea::Area(cube) => {
                let x = rng.gen_range(cube.bottom_left.x..(cube.bottom_left.x + cube.size.x));
                let y = rng.gen_range(cube.bottom_left.y..(cube.bottom_left.y + cube.size.y));
                let z = rng.gen_range(cube.bottom_left.z..(cube.bottom_left.z + cube.size.z));

                Vector3::new(x, y, z)
            }
            SpawnArea::Custom(function) => {
                function(i)
            }
        }
    }
}