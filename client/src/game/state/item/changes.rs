use crate::game::item::ItemType;
use crate::game::state::block::deserialisation::BlockStatesFile;
use crate::game::state::block::BlockStates;
use crate::game::state::item::deserialisation::ItemStatesFile;
use crate::game::state::item::ItemStates;
use crate::systems::ui::loading::LoadingUIData;
use bevy::prelude::{info, warn, AssetEvent, Assets, EventReader, ResMut};

/// Copies the itemstate asset to the Resource
pub fn track_itemstate_changes(
    mut events: EventReader<AssetEvent<ItemStatesFile>>,
    assets: ResMut<Assets<ItemStatesFile>>,
    mut states: ResMut<ItemStates>,
) {
    for event in events.read() {
        match event {
            AssetEvent::Added { .. } => {
                states.recalculate_full = true;
                states.recalculate_blocks = true;
            }
            AssetEvent::Modified { .. } => {
                states.recalculate_full = true;
                states.recalculate_blocks = true;
            }
            _ => {}
        }
    }

    if states.recalculate_full {
        info!("Reloading item states");

        // Copy data over to blockstates, with full amount of data like normals and looking up texture atlas indexes
        let (_, asset) = assets.iter().next().unwrap();

        let mut new_item_states = Vec::with_capacity(asset.states.len());

        for item in &asset.states {
            let new_item = ItemType {
                identifier: item.identifier.clone(),
                name: item.name.clone(),
                icon: item.icon.clone(),
                block_state: None,
            };

            new_item_states.push(new_item);
        }

        states.states = new_item_states;

        states.recalculate_full = false;
        info!("Built item states");
    }
}

/// Copies the blockstate indexes to the respective drops
pub fn track_blockstate_changes(
    mut events: EventReader<AssetEvent<BlockStatesFile>>,
    assets: ResMut<Assets<ItemStatesFile>>,
    mut states: ResMut<ItemStates>,
    mut block_states: ResMut<BlockStates>,
    mut loading: Option<ResMut<LoadingUIData>>,
) {
    for event in events.read() {
        match event {
            AssetEvent::Added { .. } => {
                states.recalculate_blocks = true;
            }
            AssetEvent::Modified { .. } => {
                states.recalculate_blocks = true;
            }
            _ => {}
        }
    }

    // Don't recompute until item states have been set
    if states.states.len() == 0 || block_states.states.len() == 0 {
        return;
    }

    if states.recalculate_blocks {
        info!("Reloading item block ids");

        // Copy data over to blockstates, with full amount of data like normals and looking up texture atlas indexes
        let (_, asset) = assets.iter().next().unwrap();

        let len = asset.states.len();

        for item_id in 0..len {
            if let Some((block_id, _)) = block_states
                .states
                .iter()
                .enumerate()
                .find(|(_, v)| v.identifier == asset.states.get(item_id).unwrap().block_state)
            {
                states.states.get_mut(item_id).unwrap().block_state = Some(block_id as u32);
            } else {
                let item = states.states.get_mut(item_id).unwrap();
                item.block_state = None;
                warn!(
                    "Item id '{}' tried to reference non-existant block identifier '{}'",
                    item.identifier,
                    asset.states.get(item_id).unwrap().block_state
                );
            }
        }

        states.recalculate_blocks = false;
        info!("Built item block id mapping");

        if let Some(loading) = &mut loading {
            loading.item_states = true;
        }
    }
}
