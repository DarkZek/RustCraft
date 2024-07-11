use crate::aabb::Aabb;
use crate::block::deserialisation::BlockStatesFile;
use crate::block::event::BlockStatesUpdatedEvent;
use crate::block::face::Face;
use crate::block::types::{Block, LootTableEntry};
use crate::block::{BlockStates, TEXTURE_ATLAS};
use bevy::log::{info, warn};

use crate::item::event::ItemStatesUpdatedEvent;
use crate::item::ItemStates;
use crate::viewable_direction::ViewableDirectionBitMap;
use bevy::prelude::{AssetEvent, Assets, EventReader, EventWriter, Res, ResMut};
use nalgebra::Vector3;

/// Copies the blockstate asset to the Resource
pub fn track_blockstate_changes(
    mut events: EventReader<AssetEvent<BlockStatesFile>>,
    assets: ResMut<Assets<BlockStatesFile>>,
    mut states: ResMut<BlockStates>,
    mut event_writer: EventWriter<BlockStatesUpdatedEvent>,
) {
    for event in events.read() {
        match event {
            AssetEvent::Added { .. } => {
                states.recalculate_full = true;
                states.recalculate_items = true;
            }
            AssetEvent::Modified { .. } => {
                states.recalculate_full = true;
                states.recalculate_items = true;
            }
            _ => {}
        }
    }

    // If there's no atlas we can't calculate blockstates yet. Put it off until next time
    if TEXTURE_ATLAS.get().is_some() && !TEXTURE_ATLAS.get().unwrap().exists() {
        return;
    }

    if states.recalculate_full {
        info!("Reloading block states");
        // Copy data over to blockstates, with full amount of data like normals and looking up texture atlas indexes
        let (_, asset) = assets.iter().next().unwrap();

        let mut new_block_states = Vec::with_capacity(asset.states.len());

        let error_texture = TEXTURE_ATLAS
            .get()
            .unwrap()
            .get_entry("game/error")
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
                let texture = TEXTURE_ATLAS
                    .get()
                    .unwrap()
                    .get_entry(&face.texture)
                    .unwrap_or(error_texture);

                let direction = ViewableDirectionBitMap::from_code(face.direction).unwrap();

                let normal = match direction {
                    ViewableDirectionBitMap::Top => Vector3::new(0.0, 1.0, 0.0),
                    ViewableDirectionBitMap::Bottom => Vector3::new(0.0, -1.0, 0.0),
                    ViewableDirectionBitMap::Front => Vector3::new(0.0, 0.0, -1.0),
                    ViewableDirectionBitMap::Back => Vector3::new(0.0, 0.0, 1.0),
                    ViewableDirectionBitMap::Left => Vector3::new(-1.0, 0.0, 0.0),
                    ViewableDirectionBitMap::Right => Vector3::new(1.0, 0.0, 0.0),
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

            new_block_states.push(new_block);
        }

        states.states = new_block_states;

        states.recalculate_full = false;

        info!("Built block states");

        event_writer.send(BlockStatesUpdatedEvent);
    }
}

/// Copies the items index to the Block data
pub fn track_itemstate_changes(
    mut events: EventReader<ItemStatesUpdatedEvent>,
    assets: ResMut<Assets<BlockStatesFile>>,
    mut states: ResMut<BlockStates>,
    item_states: Res<ItemStates>,
) {
    for _ in events.read() {
        states.recalculate_items = true;
    }

    // If there's no blocks we can't calculate blockstates yet. Put it off until next time
    if states.states.len() == 0 {
        return;
    }

    if states.recalculate_items {
        info!("Reloading block loot tables");
        // Copy data over to blockstates, with full amount of data like normals and looking up texture atlas indexes
        let (_, asset) = assets.iter().next().unwrap();

        let mut new_loot_table_states = Vec::with_capacity(asset.states.len());

        for block in &asset.states {
            // Convert loot table
            let mut loot_data = Vec::new();

            for drop in &block.loot_table {
                if let Some((item_id, _)) = item_states
                    .states
                    .iter()
                    .enumerate()
                    .find(|item| item.1.identifier.eq_ignore_ascii_case(&drop.item))
                {
                    loot_data.push(LootTableEntry {
                        chance: drop.chance,
                        item_id,
                    });
                } else {
                    warn!(
                        "Block {} contains invalid loot table identifier {} - Not found in type states",
                        block.identifier,
                        drop.item
                    );
                }
            }

            new_loot_table_states.push(loot_data)
        }

        states.loot_tables = new_loot_table_states;

        states.recalculate_items = false;
    }
}
