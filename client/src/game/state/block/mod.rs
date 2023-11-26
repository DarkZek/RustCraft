mod changes;
pub mod deserialisation;
mod loader;

use crate::game::blocks::{Block, LootTableEntry};
use crate::game::state::block::changes::{track_blockstate_changes, track_itemstate_changes};
use crate::game::state::block::deserialisation::BlockStatesFile;
use crate::game::state::block::loader::BlockStateAssetLoader;
use bevy::log::warn;
use bevy::prelude::{
    App, AssetApp, AssetServer, Handle, Plugin, Res, ResMut, Resource, Startup, Update,
};
use bevy::reflect::TypeUuid;

pub struct BlockStatesPlugin;

impl Plugin for BlockStatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<BlockStatesFile>()
            .init_asset_loader::<BlockStateAssetLoader>()
            .add_systems(Startup, create_block_states)
            .insert_resource(BlockStates::new())
            .add_systems(Update, track_blockstate_changes)
            .add_systems(Update, track_itemstate_changes);
    }
}

pub fn create_block_states(server: Res<AssetServer>, mut states: ResMut<BlockStates>) {
    states.asset = Some(server.load("game/state.blocks"));
}

#[derive(Debug, Clone, TypeUuid, Resource)]
#[uuid = "97103fab-1e50-36b7-0c33-0938a62b0809"]
pub struct BlockStates {
    pub states: Vec<Block>,
    pub loot_tables: Vec<Vec<LootTableEntry>>,
    /// Used to tell the blockstates to recalculate, only used when the blockstates are ready but waiting on the texture atlas to finish deserialisation
    pub recalculate_full: bool,
    /// Used to recalculate item mapping from identifier to index when items list is updated
    pub recalculate_items: bool,
    pub asset: Option<Handle<BlockStatesFile>>,
}

impl BlockStates {
    pub fn new() -> BlockStates {
        BlockStates {
            states: vec![],
            loot_tables: vec![],
            recalculate_full: false,
            recalculate_items: false,
            asset: None,
        }
    }

    // Possibly remove, keeping it because it was in old version and I might need it
    pub fn get_block(&self, i: usize) -> &Block {
        if let Some(val) = self.states.get(i) {
            val
        } else {
            warn!("Invalid block state received: {}", i);
            self.states.get(0).unwrap()
        }
    }
}
