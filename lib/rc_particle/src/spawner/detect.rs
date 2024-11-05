use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;
use crate::spawner::{ParticleSpawner, ParticleSpawnerMeta};

pub fn detect_spawner(
    query: Query<(Entity, &ParticleSpawner), Added<ParticleSpawner>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    time: Res<Time>
) {

    let spawned_at = time.elapsed_seconds();

    for (entity, spawner) in query.iter() {

        let mut mesh: Mesh = Rectangle::from_length(0.2).into();

        let mut uv_coordinates = vec![];
        uv_coordinates
            .push([spawner.texture.u_min, spawner.texture.v_min]);
        uv_coordinates
            .push([spawner.texture.u_min, spawner.texture.v_max]);
        uv_coordinates
            .push([spawner.texture.u_max, spawner.texture.v_max]);
        uv_coordinates
            .push([spawner.texture.u_max, spawner.texture.v_min]);

        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv_coordinates);

        commands
            .entity(entity)
            .insert(ParticleSpawnerMeta {
                i: 0,
                simulated_to: time.elapsed().as_nanos(),
                spawned_at,
                mesh: meshes.add(mesh)
            });
    }
}