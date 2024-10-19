use crate::systems::ui::loading::LoadingUIData;
use bevy::prelude::{AssetServer, EventReader, Res, ResMut};
use rc_shared::block::BlockStates;
use rc_shared::item::event::ItemStatesUpdatedEvent;
use rc_shared::item::ItemStates;

pub fn create_states(
    server: Res<AssetServer>,
    mut item_states: ResMut<ItemStates>,
) {
    item_states.load_states("game/state.items".to_string(), &server);
}

pub fn trigger_load_blockstates(
    mut loading: Option<ResMut<LoadingUIData>>,
    mut states: ResMut<BlockStates>,
) {
    let Some(mut loading) = loading else {
        return
    };

    // Load block states if the texture atlas has been completed and we haven't already done the block states
    if !loading.texture_atlas || loading.block_states {
        return
    }

    states.calculate_states();

    loading.block_states = true;
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
