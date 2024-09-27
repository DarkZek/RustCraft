use bevy::prelude::{EventReader, NextState, Res, ResMut};
use crate::state::AppState;
use crate::systems::chunk::builder::ChunkRebuiltEvent;
use crate::systems::chunk::ChunkSystem;
use crate::systems::chunk::flags::ChunkFlagsBitMap;

pub fn detect_connection_complete(
    packets: EventReader<ChunkRebuiltEvent>,
    mut app_state: ResMut<NextState<AppState>>,
    chunks: Res<ChunkSystem>
) {
    if packets.len() == 0 {
        return
    }

    // A chunk has been rebuilt, let's check if all chunks have been built and we're ready to rumble
    for (_, chunk) in &chunks.chunks {
        // Not ready
        if !chunk.flags.has_flag(ChunkFlagsBitMap::Ready) && !chunk.flags.has_flag(ChunkFlagsBitMap::AtEdge) {
            return
        }
    }

    // Loaded!
    app_state.set(AppState::InGame);
}