mod changes;
pub mod deserialisation;
pub mod event;
pub mod face;
mod loader;
pub mod types;

use crate::atlas::TextureAtlasTrait;
use crate::block::changes::{track_blockstate_changes, track_itemstate_changes};
use crate::block::deserialisation::BlockStatesFile;
use crate::block::event::BlockStatesUpdatedEvent;
use crate::block::loader::BlockStateAssetLoader;
use crate::block::types::{Block, LootTableEntry};
use bevy::log::warn;
use bevy::prelude::{
    App, AssetApp, AssetServer, Handle, Plugin, Res, ResMut, Resource, Startup, Update,
};
use bevy::reflect::TypeUuid;
use std::mem::MaybeUninit;
use std::sync::OnceLock;

static TEXTURE_ATLAS: OnceLock<&'static (dyn TextureAtlasTrait + Sync)> = OnceLock::new();

pub struct BlockStatesPlugin {
    pub texture_atlas: &'static (dyn TextureAtlasTrait + Sync),
}

impl Plugin for BlockStatesPlugin {
    fn build(&self, app: &mut App) {
        // Update reference
        let _ = TEXTURE_ATLAS.set(self.texture_atlas);

        app.init_asset::<BlockStatesFile>()
            .init_asset_loader::<BlockStateAssetLoader>()
            .add_event::<BlockStatesUpdatedEvent>()
            .insert_resource(BlockStates::new())
            .add_systems(Update, track_blockstate_changes)
            .add_systems(Update, track_itemstate_changes);
    }
}

#[derive(Debug, Clone, TypeUuid, Resource)]
#[uuid = "97103fab-1e50-36b7-0c33-0938a62b0809"]
pub struct BlockStates {
    pub states: Vec<Block>,
    pub loot_tables: Vec<Vec<LootTableEntry>>,
    /// Used to tell the blockstates to recalculate, only used when the blockstates are ready but waiting on the texture atlas to finish deserialisation
    pub recalculate_full: bool,
    /// Used to recalculate type mapping from identifier to index when items list is updated
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

    pub fn load_states(&mut self, path: String, asset_server: &AssetServer) {
        self.asset = Some(asset_server.load(path));
    }
}
