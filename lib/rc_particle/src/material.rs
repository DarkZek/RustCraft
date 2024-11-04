use std::time::Duration;
use bevy::prelude::*;
use nalgebra::Vector3;
use rc_shared::aabb::Aabb;
use rc_shared::atlas::{TEXTURE_ATLAS, TextureAtlasIndex};
use crate::spawner::{ParticleSpawner, SpawnArea};

#[derive(Resource)]
pub struct ParticleResource {
    pub material: Handle<StandardMaterial>,
}

pub fn setup_resource(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // Create resource
    commands.insert_resource(ParticleResource {
        material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            // base_color_texture: Some(TEXTURE_ATLAS.get().image.clone()),
            unlit: true,
            ..default()
        })
    });

    commands.spawn((
        ParticleSpawner {
            area: SpawnArea::Custom(Box::new(|i| {
                let x = i as f64 / 6.0;
                Vector3::new(x.sin() as f32, x.cos() as f32, 0.0)
            })),
            spawn_rate: 4.0,
            texture: TextureAtlasIndex::default(),
            ttl: Duration::from_millis(1500),
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
            texture: TextureAtlasIndex::default(),
            ttl: Duration::from_millis(1500),
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
            texture: TextureAtlasIndex::default(),
            ttl: Duration::from_millis(500),
        },
        Transform::from_translation(Vec3::new(-8.0, 17.0, -8.0))
    ));
}