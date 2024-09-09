use bevy::prelude::{Commands, ResMut};
use crate::systems::chunk::ChunkSystem;

pub fn on_disconnect(
    mut commands: Commands,
    mut chunk_system: ResMut<ChunkSystem>
) {
    // Unload all existing chunks
    chunk_system.unload_all_chunks(&mut commands);

    // TODO: Despawn player and everything
}