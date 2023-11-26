mod changes;
pub mod deserialisation;
mod loader;

use crate::game::item::ItemType;
use crate::game::state::item::changes::{track_blockstate_changes, track_itemstate_changes};
use crate::game::state::item::deserialisation::ItemStatesFile;
use crate::game::state::item::loader::ItemStateAssetLoader;
use bevy::prelude::{
    App, AssetApp, AssetServer, Handle, Plugin, Res, ResMut, Resource, Startup, Update,
};

pub struct ItemStatesPlugin;

impl Plugin for ItemStatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<ItemStatesFile>()
            .init_asset_loader::<ItemStateAssetLoader>()
            .add_systems(Startup, create_block_states)
            .insert_resource(ItemStates::new())
            .add_systems(Update, track_itemstate_changes)
            .add_systems(Update, track_blockstate_changes);
    }
}

pub fn create_block_states(server: Res<AssetServer>, mut states: ResMut<ItemStates>) {
    states.asset = Some(server.load("game/state.items"));
}

#[derive(Resource)]
pub struct ItemStates {
    pub states: Vec<ItemType>,
    /// Recalculate all item types from source asset
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
}
