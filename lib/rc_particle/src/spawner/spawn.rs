
use std::time::Duration;
use bevy::prelude::*;
use rc_shared::helpers::to_bevy_vec3;
use crate::material::ParticleResource;
use crate::particle::Particle;
use crate::spawner::{ParticleSpawner, ParticleSpawnerMeta};
use crate::spawner::simulation::ParticleSimulationData;

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

            translation += to_bevy_vec3(spawner.area.get_offset(spawner_meta.i));

            let transform = Transform::from_translation(translation);

            let mut entity_commands = commands.spawn(
                PbrBundle {
                    mesh: spawner_meta.mesh.clone(),
                    material: resource.material.clone(),
                    transform,
                    ..default()
                }
            );
            entity_commands.insert(Particle {
                ttl: spawner.particle_ttl.clone(),
                created
            });

            if let Some(sim_settings) = &spawner.simulation {
                entity_commands.insert(ParticleSimulationData {
                    velocity: sim_settings.initial_velocity.clone(),
                    settings: *sim_settings
                });
            }

            spawner_meta.i += 1;
        }
    }
}