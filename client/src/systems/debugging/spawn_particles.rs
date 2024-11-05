use std::time::Duration;
use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;
use bevy::math::Vec3;
use bevy::prelude::{Commands, EventReader, KeyCode, Transform};
use nalgebra::Vector3;
use rc_particle::{ParticleSimulationSettings, ParticleSpawner, SpawnArea};
use rc_shared::aabb::Aabb;
use rc_shared::atlas::TEXTURE_ATLAS;

pub fn spawn_particles(
    mut commands: Commands,
    mut evr_kbd: EventReader<KeyboardInput>,
) {
    let mut spawn = false;
    for event in evr_kbd.read() {
        if event.key_code == KeyCode::F7 && event.state == ButtonState::Pressed {
            spawn = true;
        }
    }

    if !spawn {
        return;
    }

    commands.spawn((
        ParticleSpawner {
            area: SpawnArea::Custom(Box::new(|i| {
                let x = i as f64 / 6.0;
                Vector3::new(x.sin() as f32, x.cos() as f32, 0.0)
            })),
            spawn_rate: 4.0,
            texture: *TEXTURE_ATLAS.get().index.get("game/stone").unwrap(),
            particle_ttl: Duration::from_millis(1500),
            expires: Duration::from_millis(5000),
            simulation: None
        },
        Transform::from_translation(Vec3::new(-5.0, 17.0, -5.0))
    ));

    commands.spawn((
        ParticleSpawner {
            area: SpawnArea::Custom(Box::new(|i| {
                let x = i as f64 / 80.0;
                Vector3::new(x.sin() as f32, x.cos() as f32, 0.0)
            })),
            spawn_rate: 100.0,
            texture: *TEXTURE_ATLAS.get().index.get("game/dirt").unwrap(),
            particle_ttl: Duration::from_millis(1500),
            expires: Duration::from_millis(5000),
            simulation: None
        },
        Transform::from_translation(Vec3::new(-8.0, 17.0, -5.0))
    ));

    commands.spawn((
        ParticleSpawner {
            area: SpawnArea::Area(Aabb::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(2.0, 2.0, 2.0),
            )),
            spawn_rate: 10.0,
            texture: *TEXTURE_ATLAS.get().index.get("game/wood_top").unwrap(),
            particle_ttl: Duration::from_millis(1200),
            expires: Duration::from_millis(5000),
            simulation: Some(ParticleSimulationSettings {
                gravity_strength: 4.0,
                has_gravity: true,
                initial_velocity: Vector3::new(0.0, 2.5, 0.0),
                acceleration: Vector3::new(0.0, 0.0, 0.2),
                drag: 0.01,
            })
        },
        Transform::from_translation(Vec3::new(-8.0, 17.0, -8.0))
    ));
}