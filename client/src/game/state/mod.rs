use crate::systems::chunk::builder::{RerenderChunkFlag, RerenderChunkFlagContext};
use crate::systems::chunk::ChunkSystem;
use crate::systems::ui::loading::LoadingUIData;
use bevy::prelude::{AssetServer, EventReader, EventWriter, Res, ResMut};
use rc_shared::block::event::BlockStatesUpdatedEvent;
use rc_shared::block::BlockStates;
use rc_shared::item::event::ItemStatesUpdatedEvent;
use rc_shared::item::ItemStates;

pub fn create_states(
    server: Res<AssetServer>,
    mut block_states: ResMut<BlockStates>,
    mut item_states: ResMut<ItemStates>,
) {
    block_states.load_states("game/state.blocks".to_string(), &server);
    item_states.load_states("game/state.items".to_string(), &server);
}

pub fn track_blockstate_changes(
    event: EventReader<BlockStatesUpdatedEvent>,
    loading: Option<ResMut<LoadingUIData>>,
    chunks: ResMut<ChunkSystem>,
    mut rerender_chunks: EventWriter<RerenderChunkFlag>,
) {
    if event.is_empty() {
        return;
    }

    // Rerender all chunks with new block states
    for (pos, _chunk) in &chunks.chunks {
        rerender_chunks.send(RerenderChunkFlag {
            chunk: *pos,
            context: RerenderChunkFlagContext::None,
        });
    }

    if let Some(mut loading) = loading {
        loading.block_states = true;
    }
}

pub fn track_itemstate_changes(
    event: EventReader<ItemStatesUpdatedEvent>,
    loading: Option<ResMut<LoadingUIData>>,
) {
    if event.is_empty() {
        return;
    }

    if let Some(mut loading) = loading {
        loading.item_states = true;
    }
}
