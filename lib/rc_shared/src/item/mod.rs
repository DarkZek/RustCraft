mod changes;
pub mod deserialisation;
pub mod event;
mod loader;
pub mod types;

use crate::item::changes::{track_blockstate_changes, track_itemstate_changes};
use crate::item::deserialisation::ItemStatesFile;
use crate::item::event::ItemStatesUpdatedEvent;
use crate::item::loader::ItemStateAssetLoader;
use crate::item::types::ItemType;
use bevy::prelude::{
    App, AssetApp, AssetServer, Handle, Plugin, Resource, Update,
};

pub struct ItemStatesPlugin;

impl Plugin for ItemStatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<ItemStatesFile>()
            .init_asset_loader::<ItemStateAssetLoader>()
            .add_event::<ItemStatesUpdatedEvent>()
            .insert_resource(ItemStates::new())
            .add_systems(Update, track_itemstate_changes)
            .add_systems(Update, track_blockstate_changes);
    }
}

#[derive(Resource)]
pub struct ItemStates {
    pub states: Vec<ItemType>,
    /// Recalculate all type types from source asset
    pub recalculate_full: bool,
    /// Recalculate block id's from identifiers
    pub recalculate_blocks: bool,
    pub asset: Option<Handle<ItemStatesFile>>,
}

impl ItemStates {
    pub fn new() -> ItemStates {
        ItemStates {
            states: vec![],
            recalculate_full: false,
            recalculate_blocks: false,
            asset: None,
        }
    }

    pub fn load_states(&mut self, path: String, asset_server: &AssetServer) {
        self.asset = Some(asset_server.load(path));
    }

    pub fn get_by_id(&self, name: &str) -> Option<(usize, &ItemType)> {
        for (i, state) in self.states.iter().enumerate() {
            if state.identifier == name {
                return Some((i, state));
            }
        }

        None
    }
}
