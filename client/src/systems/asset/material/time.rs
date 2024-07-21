use bevy::asset::Assets;
use bevy::prelude::{Res, ResMut, Time};
use crate::systems::asset::AssetService;
use crate::systems::asset::material::translucent_chunk_extension::TranslucentChunkMaterial;

pub fn update_time(
    time: Res<Time>,
    service: ResMut<AssetService>,
    mut materials: ResMut<Assets<TranslucentChunkMaterial>>,
) {
    materials.get_mut(&service.translucent_texture_atlas_material).unwrap().extension.time
        += time.delta_seconds();
}