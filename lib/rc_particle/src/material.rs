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

pub fn setup_resource_with_atlas(
    mut resource: ResMut<ParticleResource>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let material = materials.get_mut(&resource.material).unwrap();

    material.base_color_texture = Some(TEXTURE_ATLAS.get().image.clone());
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
}