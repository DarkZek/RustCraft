use bevy::prelude::*;
use crate::spawner::{ParticleSpawner, ParticleSpawnerMeta};

pub fn detect_spawner(
    query: Query<Entity, Added<ParticleSpawner>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    time: Res<Time>
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(ParticleSpawnerMeta {
                i: 0,
                simulated_to: time.elapsed().as_nanos(),
                mesh: meshes.add(Cuboid::from_length(0.2))
            });
    }
}