use bevy::prelude::*;
use crate::particle::Particle;
use crate::ParticleSpawner;
use crate::spawner::ParticleSpawnerMeta;

pub fn do_expire(
    query: Query<(Entity, &ParticleSpawner, &ParticleSpawnerMeta)>,
    mut commands: Commands,
    time: Res<Time>
) {

    let now = time.elapsed_seconds();

    for (entity, particle, meta) in query.iter() {

        let duration = particle.expires.as_secs_f32();

        if now < duration + meta.spawned_at {
            continue;
        }

        commands.entity(entity).despawn();
    }
}
