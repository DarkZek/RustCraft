use crate::block::{BlockDefinitionIndex, BlockStates};
use crate::item::deserialisation::ItemStatesFile;
use crate::item::event::ItemStatesUpdatedEvent;
use crate::item::types::ItemType;
use crate::item::ItemStates;
use bevy::log::{info, warn};
use bevy::prelude::{AssetEvent, Assets, EventReader, EventWriter, ResMut};
use crate::block::definition::BLOCK_DEFINITIONS;

/// Copies the itemstate asset to the Resource
pub fn track_itemstate_changes(
    mut events: EventReader<AssetEvent<ItemStatesFile>>,
    assets: ResMut<Assets<ItemStatesFile>>,
    mut states: ResMut<ItemStates>,
    mut event_writer: EventWriter<ItemStatesUpdatedEvent>,
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
        info!("Reloading type states");

        // Copy data over to blockstates, with full amount of data like normals and looking up texture atlas indexes
        let (_, asset) = assets.iter().next().unwrap();

        let mut new_item_states = Vec::with_capacity(asset.states.len());

        for item in &asset.states {
            let new_item = ItemType {
                identifier: item.identifier.clone(),
                name: item.name.clone(),
                icon: item.icon.clone(),
                block_definition_index: None,
            };

            new_item_states.push(new_item);
        }

        states.states = new_item_states;

        states.recalculate_full = false;
        info!("Built type states");

        event_writer.send(ItemStatesUpdatedEvent);
    }
}

/// Copies the blockstate indexes to the respective drops
pub fn track_blockstate_changes(
    assets: ResMut<Assets<ItemStatesFile>>,
    mut states: ResMut<ItemStates>,
    block_states: ResMut<BlockStates>,
) {
    // Don't recompute until type states have been set
    if states.states.len() == 0 || block_states.block_index.len() == 0 {
        return;
    }

    if states.recalculate_blocks {
        info!("Reloading type block ids");

        // Copy data over to itemstates, with full amount of data like normals and looking up texture atlas indexes
        let (_, asset) = assets.iter().next().unwrap();

        let len = asset.states.len();

        for item_id in 0..len {
            if let Some((definition_index, _)) = BLOCK_DEFINITIONS
                .get()
                .unwrap()
                .iter()
                .enumerate()
                .find(|(_, v)| v.identifier == asset.states.get(item_id).unwrap().block_state)
            {
                states.states.get_mut(item_id).unwrap().block_definition_index = Some(BlockDefinitionIndex(definition_index));
            } else {
                let item = states.states.get_mut(item_id).unwrap();
                item.block_definition_index = None;
                warn!(
                    "Item id '{}' tried to reference non-existent block identifier '{}'",
                    item.identifier,
                    asset.states.get(item_id).unwrap().block_state
                );
            }
        }

        states.recalculate_blocks = false;
        info!("Built type block id mapping");
    }
}
