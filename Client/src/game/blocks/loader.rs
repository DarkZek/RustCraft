use crate::game::blocks::loading::BlockStatesFile;
use crate::game::blocks::states::BlockStates;
use crate::game::blocks::Block;
use crate::game::mesh::face::Face;
use crate::game::viewable_direction::ViewableDirectionBitMap;

use crate::services::asset::AssetService;
use crate::services::chunk::systems::mesh_builder::RerenderChunkFlag;
use crate::services::chunk::ChunkService;
use crate::state::AppState;
use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use bevy::prelude::*;
use nalgebra::Vector3;

#[derive(Default)]
pub struct BlockStateAssetLoader;

impl AssetLoader for BlockStateAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let states = match serde_json::from_slice::<BlockStatesFile>(bytes) {
                Ok(val) => val,
                Err(e) => panic!("Invalid block states json {:?}", e), // TODO: Handle this better
            };

            load_context.set_default_asset(LoadedAsset::new(states));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["blocks"]
    }
}

// TODO: Set the block states files contents to nothing after copying to save RAM

/// Copies the blockstate asset to the Resource
pub fn track_blockstate_changes(
    mut events: EventReader<AssetEvent<BlockStatesFile>>,
    assets: ResMut<Assets<BlockStatesFile>>,
    mut states: ResMut<BlockStates>,
    atlas: Res<AssetService>,
    mut app_state: ResMut<State<AppState>>,
    chunks: ResMut<ChunkService>,
    mut commands: Commands,
) {
    for event in events.iter() {
        match event {
            AssetEvent::Created { .. } => {
                states.recalculate = true;
            }
            AssetEvent::Modified { .. } => {
                states.recalculate = true;
            }
            AssetEvent::Removed { .. } => {}
        }
    }

    // If there's no atlas we can't calculate blockstates yet. Put it off until next time
    if atlas.texture_atlas.is_none() {
        return;
    }

    if states.recalculate {
        // Copy data over to blockstates, with full amount of data like normals and looking up texture atlas indexes
        let (_, asset) = assets.iter().next().unwrap();

        let mut new_states = Vec::with_capacity(asset.states.len());

        let error_texture = *atlas
            .texture_atlas
            .as_ref()
            .unwrap()
            .index
            .get("game/invalid")
            .unwrap();

        for block in &asset.states {
            let mut new_block = Block {
                identifier: block.identifier.clone(),
                translucent: block.translucent,
                full: block.full,
                draw_betweens: block.draw_betweens,
                faces: Vec::with_capacity(block.faces.len()),
            };

            for face in &block.faces {
                // Lookup atlas index, or display glitch texture
                let texture = *atlas
                    .texture_atlas
                    .as_ref()
                    .unwrap()
                    .index
                    .get(&face.texture)
                    .unwrap_or(&error_texture);

                let direction = ViewableDirectionBitMap::from_code(face.direction).unwrap();

                let normal = match direction {
                    ViewableDirectionBitMap::Top => Vector3::new(0.0, 1.0, 0.0),
                    ViewableDirectionBitMap::Bottom => Vector3::new(0.0, -1.0, 0.0),
                    ViewableDirectionBitMap::Left => Vector3::new(0.0, 0.0, 1.0),
                    ViewableDirectionBitMap::Right => Vector3::new(0.0, 0.0, -1.0),
                    ViewableDirectionBitMap::Front => Vector3::new(1.0, 0.0, 0.0),
                    ViewableDirectionBitMap::Back => Vector3::new(-1.0, 0.0, 0.0),
                };

                new_block.faces.push(Face {
                    top_left: face.top_left,
                    size: face.size,
                    texture,
                    normal,
                    edge: face.edge,
                    direction,
                })
            }

            new_states.push(new_block);
        }

        states.states = new_states;

        states.recalculate = false;
        info!("Built block states");

        // Rerender all chunks with new block states
        for (pos, chunk) in &chunks.chunks {
            commands
                .entity(chunk.entity)
                .insert(RerenderChunkFlag { chunk: *pos });
        }
    }

    // If we're still in loading mode, the block states being loaded means we're ready for the main menu. This may be changed in the future
    if app_state.current() == &AppState::Loading {
        app_state.set(AppState::MainMenu).unwrap();
    }
}
