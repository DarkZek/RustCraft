
use std::time::Duration;
use bevy::prelude::*;
use crate::material::ParticleResource;
use crate::particle::Particle;
use crate::spawner::{ParticleSpawner, ParticleSpawnerMeta};

pub fn do_spawn(
    mut query: Query<(&Transform, &ParticleSpawner, &mut ParticleSpawnerMeta)>,
    mut commands: Commands,
    resource: Res<ParticleResource>,
    time: Res<Time>
) {

    let created = time.elapsed();
    let target_simulated_time = created.as_nanos();

    for (transform, spawner, mut spawner_meta) in query.iter_mut() {

        // Calculate how many particles we should spawn this frame
        let second_percent = 1.0/spawner.spawn_rate;
        let ns_delay_between_particles = (second_percent as f64 * Duration::from_secs(1).as_nanos() as f64) as u128;

        while target_simulated_time > spawner_meta.simulated_to {

            spawner_meta.simulated_to += ns_delay_between_particles;

            // Spawn
            let mut translation: Vec3 = transform.translation;

            let x = created.as_nanos() as f64 / 1_000_000_000.0;

            translation += Vec3::new(x.sin() as f32, x.cos() as f32, 0.0);

            let transform = Transform::from_translation(translation);

            commands.spawn(
                PbrBundle {
                    mesh: spawner_meta.mesh.clone(),
                    material: resource.material.clone(),
                    transform,
                    ..default()
                }
            ).insert(Particle {
                ttl: spawner.ttl.clone(),
                created
            });
        }
    }
}