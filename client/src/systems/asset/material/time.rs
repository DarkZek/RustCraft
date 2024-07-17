use bevy::asset::Assets;
use bevy::prelude::{Res, ResMut, Time};
use crate::systems::asset::AssetService;
use crate::systems::asset::material::chunk_extension::ChunkMaterial;

pub fn update_time(
    time: Res<Time>,
    service: ResMut<AssetService>,
    mut materials: ResMut<Assets<ChunkMaterial>>,
) {
    materials.get_mut(&service.translucent_texture_atlas_material).unwrap().extension.time
        += time.delta_seconds();
}