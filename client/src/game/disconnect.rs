use bevy::prelude::{Commands, Entity, Or, Query, ResMut, With};
use rc_shared::game_objects::{DebugGameObjectData, ItemDropGameObjectData, PlayerGameObjectData};
use crate::systems::chunk::ChunkSystem;

pub fn on_disconnect(
    mut commands: Commands,
    mut chunk_system: ResMut<ChunkSystem>,
    mut entities: Query<Entity, Or<(With<PlayerGameObjectData>, With<ItemDropGameObjectData>, With<DebugGameObjectData>)>>
) {
    // Unload all existing chunks
    chunk_system.unload_all_chunks(&mut commands);

    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}