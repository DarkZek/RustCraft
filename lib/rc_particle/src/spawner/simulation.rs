use bevy::prelude::*;
use nalgebra::Vector3;
use rc_shared::helpers::to_bevy_vec3;
use crate::ParticleSpawner;
use crate::spawner::ParticleSpawnerMeta;

#[derive(Copy, Clone, Debug)]
pub struct ParticleSimulationSettings {
    pub has_gravity: bool,
    pub gravity_strength: f32,
    pub initial_velocity: Vector3<f32>,
    pub acceleration: Vector3<f32>,
    pub drag: f32
}

#[derive(Component)]
pub struct ParticleSimulationData {
    pub settings: ParticleSimulationSettings,
    pub velocity: Vector3<f32>
}

pub fn do_simulation(
    mut query: Query<(&mut Transform, &mut ParticleSimulationData)>,
    time: Res<Time>
) {
    let delta_time = time.delta().as_secs_f32();

    for (mut transform, mut sim_data) in query.iter_mut() {

        let mut velocity = sim_data.velocity;

        if sim_data.settings.has_gravity {
            velocity += Vector3::new(0.0, -1.0, 0.0) * delta_time * sim_data.settings.gravity_strength;
        }


        velocity += sim_data.settings.acceleration * sim_data.settings.gravity_strength * delta_time;

        velocity = velocity * (1.0-(sim_data.settings.drag * delta_time));

        sim_data.velocity = velocity;

        // Adjust position
        transform.translation += to_bevy_vec3(sim_data.velocity) * delta_time;
    }
}