#![feature(trivial_bounds)]
mod spawner;
mod particle;
mod material;

pub use material::ParticleResource;
pub use material::setup_resource_with_atlas;
pub use spawner::ParticleSpawner;
pub use spawner::SpawnArea;

use bevy::prelude::*;
use crate::material::setup_resource;
use crate::spawner::spawn::do_spawn;
use crate::spawner::despawn::do_despawn;
use crate::spawner::detect::detect_spawner;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_resource)
            .add_systems(Update, (do_spawn, do_despawn, detect_spawner));
    }
}