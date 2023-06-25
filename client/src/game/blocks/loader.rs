use crate::game::blocks::loading::BlockStatesFile;
use crate::game::blocks::states::BlockStates;
use crate::game::blocks::Block;
use crate::game::viewable_direction::ViewableDirectionBitMap;

use crate::systems::asset::AssetService;
use crate::systems::chunk::ChunkSystem;
use crate::systems::physics::aabb::Aabb;
use crate::systems::ui::loading::LoadingData;

use crate::systems::chunk::builder::{RerenderChunkFlag, RerenderChunkFlagContext};
use crate::systems::chunk::mesh::face::Face;
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
    chunks: ResMut<ChunkSystem>,
    mut commands: Commands,
    mut loading: ResMut<LoadingData>,
    mut rerender_chunks: EventWriter<RerenderChunkFlag>,
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
        info!("Reloading block states");
        // Copy data over to blockstates, with full amount of data like normals and looking up texture atlas indexes
        let (_, asset) = assets.iter().next().unwrap();

        let mut new_states = Vec::with_capacity(asset.states.len());

        let error_texture = *atlas
            .texture_atlas
            .as_ref()
            .unwrap()
            .index
            .get("game/error")
            .unwrap();

        for block in &asset.states {
            let mut new_block = Block {
                identifier: block.identifier.clone(),
                translucent: block.translucent,
                full: block.full,
                draw_betweens: block.draw_betweens,
                faces: Vec::with_capacity(block.faces.len()),
                collision_boxes: (&block.colliders)
                    .iter()
                    .filter(|v| v.collidable)
                    .map(|v| Aabb::new(v.bottom_left, v.size))
                    .collect::<Vec<Aabb>>(),
                bounding_boxes: (&block.colliders)
                    .iter()
                    .map(|v| Aabb::new(v.bottom_left, v.size))
                    .collect::<Vec<Aabb>>(),
                emission: block.emission,
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
                    ViewableDirectionBitMap::Left => Vector3::new(0.0, 0.0, -1.0),
                    ViewableDirectionBitMap::Right => Vector3::new(0.0, 0.0, 1.0),
                    ViewableDirectionBitMap::Front => Vector3::new(-1.0, 0.0, 0.0),
                    ViewableDirectionBitMap::Back => Vector3::new(1.0, 0.0, 0.0),
                };

                new_block.faces.push(Face {
                    top_left: face.top_left,
                    top_right: face.top_right,
                    bottom_left: face.bottom_left,
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
            rerender_chunks.send(RerenderChunkFlag {
                chunk: *pos,
                context: RerenderChunkFlagContext::None,
            });
        }

        loading.block_states = true;
    }
}
