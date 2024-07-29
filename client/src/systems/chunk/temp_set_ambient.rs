use bevy::asset::Assets;
use bevy::prelude::{Query, ResMut, Transform, With};
use nalgebra::clamp;
use crate::game::player::Player;
use crate::systems::asset::AssetService;
use crate::systems::asset::material::chunk_extension::ChunkMaterial;
use crate::systems::asset::material::translucent_chunk_extension::TranslucentChunkMaterial;

// While there is no sunlight propagation, the surface is entirely without light which looks wacky.
// For now, just bump the ambient light while on the surface
pub fn temp_set_ambient(
    asset_service: ResMut<AssetService>,
    mut materials: ResMut<Assets<ChunkMaterial>>,
    mut translucent_materials: ResMut<Assets<TranslucentChunkMaterial>>,
    player_pos: Query<&Transform, With<Player>>
) {

    let position = player_pos.get_single().unwrap();

    // Start increasing once you hit y=5
    let ambient_strength = if position.translation.y > 5.0 {
        1.0
    } else {
        0.2
    };

    materials.get_mut(&asset_service.opaque_texture_atlas_material).unwrap().extension.ambient_strength = ambient_strength;
    translucent_materials.get_mut(&asset_service.translucent_texture_atlas_material).unwrap().extension.ambient_strength = ambient_strength;
}