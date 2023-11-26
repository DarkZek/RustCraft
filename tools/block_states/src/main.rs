use crate::loading::{
    BlockStatesFile, DeserialisedAabb, DeserialisedBlock, DeserialisedFace,
    DeserialisedLootTableEntry,
};
use crate::pipe::pipe;
use nalgebra::Vector3;
use std::fs;
use std::ops::Add;

pub mod loading;
mod pipe;

fn main() {
    let mut states = vec![
        DeserialisedBlock {
            identifier: "mcv3::Air".to_string(),
            translucent: true,
            full: false,
            draw_betweens: false,
            faces: vec![],
            colliders: vec![],
            emission: [0, 0, 0, 0],
            loot_table: vec![],
        },
        DeserialisedBlock {
            identifier: "mcv3::Dirt".to_string(),
            translucent: false,
            full: true,
            draw_betweens: false,
            faces: vec![
                DeserialisedFace {
                    top_left: Vector3::new(0.0, 1.0, 0.0),
                    top_right: Vector3::new(1.0, 1.0, 0.0),
                    bottom_left: Vector3::new(0.0, 1.0, 1.0),
                    texture: "game/dirt".to_string(),
                    direction: 1,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(0.0, 0.0, 0.0),
                    top_right: Vector3::new(0.0, 0.0, 1.0),
                    bottom_left: Vector3::new(1.0, 0.0, 0.0),
                    texture: "game/dirt".to_string(),
                    direction: 2,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(0.0, 0.0, 0.0),
                    top_right: Vector3::new(0.0, 1.0, 0.0),
                    bottom_left: Vector3::new(0.0, 0.0, 1.0),
                    texture: "game/dirt".to_string(),
                    direction: 4,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(1.0, 0.0, 1.0),
                    top_right: Vector3::new(1.0, 1.0, 1.0),
                    bottom_left: Vector3::new(1.0, 0.0, 0.0),
                    texture: "game/dirt".to_string(),
                    direction: 8,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(1.0, 0.0, 0.0),
                    top_right: Vector3::new(1.0, 1.0, 0.0),
                    bottom_left: Vector3::new(0.0, 0.0, 0.0),
                    texture: "game/dirt".to_string(),
                    direction: 16,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(0.0, 0.0, 1.0),
                    top_right: Vector3::new(0.0, 1.0, 1.0),
                    bottom_left: Vector3::new(1.0, 0.0, 1.0),
                    texture: "game/dirt".to_string(),
                    direction: 32,
                    edge: true,
                },
            ],
            colliders: vec![DeserialisedAabb {
                bottom_left: Vector3::new(0.0, 0.0, 0.0),
                size: Vector3::new(1.0, 1.0, 1.0),
                collidable: true,
            }],
            emission: [0, 0, 0, 0],
            loot_table: vec![DeserialisedLootTableEntry {
                chance: 1.0,
                item: "mcv3::DirtBlockItem".to_string(),
            }],
        },
        DeserialisedBlock {
            identifier: "mcv3::Grass".to_string(),
            translucent: false,
            full: true,
            draw_betweens: false,
            faces: vec![
                DeserialisedFace {
                    top_left: Vector3::new(0.0, 1.0, 0.0),
                    top_right: Vector3::new(1.0, 1.0, 0.0),
                    bottom_left: Vector3::new(0.0, 1.0, 1.0),
                    texture: "game/grass_top".to_string(),
                    direction: 1,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(0.0, 0.0, 0.0),
                    top_right: Vector3::new(0.0, 0.0, 1.0),
                    bottom_left: Vector3::new(1.0, 0.0, 0.0),
                    texture: "game/dirt".to_string(),
                    direction: 2,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(0.0, 0.0, 0.0),
                    top_right: Vector3::new(0.0, 1.0, 0.0),
                    bottom_left: Vector3::new(0.0, 0.0, 1.0),
                    texture: "game/grass_side".to_string(),
                    direction: 4,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(1.0, 0.0, 1.0),
                    top_right: Vector3::new(1.0, 1.0, 1.0),
                    bottom_left: Vector3::new(1.0, 0.0, 0.0),
                    texture: "game/grass_side".to_string(),
                    direction: 8,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(1.0, 0.0, 0.0),
                    top_right: Vector3::new(1.0, 1.0, 0.0),
                    bottom_left: Vector3::new(0.0, 0.0, 0.0),
                    texture: "game/grass_side".to_string(),
                    direction: 16,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(0.0, 0.0, 1.0),
                    top_right: Vector3::new(0.0, 1.0, 1.0),
                    bottom_left: Vector3::new(1.0, 0.0, 1.0),
                    texture: "game/grass_side".to_string(),
                    direction: 32,
                    edge: true,
                },
            ],
            colliders: vec![DeserialisedAabb {
                bottom_left: Vector3::new(0.0, 0.0, 0.0),
                size: Vector3::new(1.0, 1.0, 1.0),
                collidable: true,
            }],
            emission: [0, 0, 0, 0],
            loot_table: vec![DeserialisedLootTableEntry {
                chance: 1.0,
                item: "mcv3::DirtBlockItem".to_string(),
            }],
        },
        DeserialisedBlock {
            identifier: "mcv3::LongGrass".to_string(),
            translucent: true,
            full: false,
            draw_betweens: true,
            faces: vec![
                DeserialisedFace {
                    top_left: Vector3::new(0.0, 0.0, 0.0),
                    top_right: Vector3::new(0.0, 1.0, 0.0),
                    bottom_left: Vector3::new(1.0, 0.0, 1.0),
                    texture: "game/long_grass".to_string(),
                    direction: 1,
                    edge: false,
                },
                DeserialisedFace {
                    top_left: Vector3::new(1.0, 0.0, 1.0),
                    top_right: Vector3::new(1.0, 1.0, 1.0),
                    bottom_left: Vector3::new(0.0, 0.0, 0.0),
                    texture: "game/long_grass".to_string(),
                    direction: 1,
                    edge: false,
                },
                DeserialisedFace {
                    top_left: Vector3::new(0.0, 0.0, 1.0),
                    top_right: Vector3::new(0.0, 1.0, 1.0),
                    bottom_left: Vector3::new(1.0, 0.0, 0.0),
                    texture: "game/long_grass".to_string(),
                    direction: 1,
                    edge: false,
                },
                DeserialisedFace {
                    top_left: Vector3::new(1.0, 0.0, 0.0),
                    top_right: Vector3::new(1.0, 1.0, 0.0),
                    bottom_left: Vector3::new(0.0, 0.0, 1.0),
                    texture: "game/long_grass".to_string(),
                    direction: 1,
                    edge: false,
                },
            ],
            colliders: vec![DeserialisedAabb {
                bottom_left: Vector3::new(0.2, 0.0, 0.2),
                size: Vector3::new(0.6, 0.8, 0.6),
                collidable: false,
            }],
            emission: [0, 0, 0, 0],
            loot_table: vec![],
        },
        DeserialisedBlock {
            identifier: "mcv3::Wood".to_string(),
            translucent: false,
            full: true,
            draw_betweens: false,
            faces: vec![
                DeserialisedFace {
                    top_left: Vector3::new(0.0, 1.0, 0.0),
                    top_right: Vector3::new(1.0, 1.0, 0.0),
                    bottom_left: Vector3::new(0.0, 1.0, 1.0),
                    texture: "game/wood_log".to_string(),
                    direction: 1,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(0.0, 0.0, 0.0),
                    top_right: Vector3::new(0.0, 0.0, 1.0),
                    bottom_left: Vector3::new(1.0, 0.0, 0.0),
                    texture: "game/wood_log".to_string(),
                    direction: 2,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(0.0, 0.0, 0.0),
                    top_right: Vector3::new(0.0, 1.0, 0.0),
                    bottom_left: Vector3::new(0.0, 0.0, 1.0),
                    texture: "game/wood_log".to_string(),
                    direction: 4,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(1.0, 0.0, 1.0),
                    top_right: Vector3::new(1.0, 1.0, 1.0),
                    bottom_left: Vector3::new(1.0, 0.0, 0.0),
                    texture: "game/wood_log".to_string(),
                    direction: 8,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(1.0, 0.0, 0.0),
                    top_right: Vector3::new(1.0, 1.0, 0.0),
                    bottom_left: Vector3::new(0.0, 0.0, 0.0),
                    texture: "game/wood_log".to_string(),
                    direction: 16,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(0.0, 0.0, 1.0),
                    top_right: Vector3::new(0.0, 1.0, 1.0),
                    bottom_left: Vector3::new(1.0, 0.0, 1.0),
                    texture: "game/wood_log".to_string(),
                    direction: 32,
                    edge: true,
                },
            ],
            colliders: vec![DeserialisedAabb {
                bottom_left: Vector3::new(0.0, 0.0, 0.0),
                size: Vector3::new(1.0, 1.0, 1.0),
                collidable: true,
            }],
            emission: [0, 0, 0, 0],
            loot_table: vec![DeserialisedLootTableEntry {
                chance: 1.0,
                item: "mcv3::WoodItem".to_string(),
            }],
        },
        DeserialisedBlock {
            identifier: "mcv3::Leaves".to_string(),
            translucent: true,
            full: true,
            draw_betweens: true,
            faces: vec![
                DeserialisedFace {
                    top_left: Vector3::new(0.0, 1.0, 0.0),
                    top_right: Vector3::new(1.0, 1.0, 0.0),
                    bottom_left: Vector3::new(0.0, 1.0, 1.0),
                    texture: "game/tree_leaves".to_string(),
                    direction: 1,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(0.0, 0.0, 0.0),
                    top_right: Vector3::new(0.0, 0.0, 1.0),
                    bottom_left: Vector3::new(1.0, 0.0, 0.0),
                    texture: "game/tree_leaves".to_string(),
                    direction: 2,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(0.0, 0.0, 0.0),
                    top_right: Vector3::new(0.0, 1.0, 0.0),
                    bottom_left: Vector3::new(0.0, 0.0, 1.0),
                    texture: "game/tree_leaves".to_string(),
                    direction: 4,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(1.0, 0.0, 1.0),
                    top_right: Vector3::new(1.0, 1.0, 1.0),
                    bottom_left: Vector3::new(1.0, 0.0, 0.0),
                    texture: "game/tree_leaves".to_string(),
                    direction: 8,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(1.0, 0.0, 0.0),
                    top_right: Vector3::new(1.0, 1.0, 0.0),
                    bottom_left: Vector3::new(0.0, 0.0, 0.0),
                    texture: "game/tree_leaves".to_string(),
                    direction: 16,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(0.0, 0.0, 1.0),
                    top_right: Vector3::new(0.0, 1.0, 1.0),
                    bottom_left: Vector3::new(1.0, 0.0, 1.0),
                    texture: "game/tree_leaves".to_string(),
                    direction: 32,
                    edge: true,
                },
            ],
            colliders: vec![DeserialisedAabb {
                bottom_left: Vector3::new(0.0, 0.0, 0.0),
                size: Vector3::new(1.0, 1.0, 1.0),
                collidable: true,
            }],
            emission: [0, 0, 0, 0],
            loot_table: vec![],
        },
        DeserialisedBlock {
            identifier: "mcv3::Stone".to_string(),
            translucent: false,
            full: true,
            draw_betweens: false,
            faces: vec![
                DeserialisedFace {
                    top_left: Vector3::new(0.0, 1.0, 0.0),
                    top_right: Vector3::new(1.0, 1.0, 0.0),
                    bottom_left: Vector3::new(0.0, 1.0, 1.0),
                    texture: "game/stone".to_string(),
                    direction: 1,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(0.0, 0.0, 0.0),
                    top_right: Vector3::new(0.0, 0.0, 1.0),
                    bottom_left: Vector3::new(1.0, 0.0, 0.0),
                    texture: "game/stone".to_string(),
                    direction: 2,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(0.0, 0.0, 0.0),
                    top_right: Vector3::new(0.0, 1.0, 0.0),
                    bottom_left: Vector3::new(0.0, 0.0, 1.0),
                    texture: "game/stone".to_string(),
                    direction: 4,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(1.0, 0.0, 1.0),
                    top_right: Vector3::new(1.0, 1.0, 1.0),
                    bottom_left: Vector3::new(1.0, 0.0, 0.0),
                    texture: "game/stone".to_string(),
                    direction: 8,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(1.0, 0.0, 0.0),
                    top_right: Vector3::new(1.0, 1.0, 0.0),
                    bottom_left: Vector3::new(0.0, 0.0, 0.0),
                    texture: "game/stone".to_string(),
                    direction: 16,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(0.0, 0.0, 1.0),
                    top_right: Vector3::new(0.0, 1.0, 1.0),
                    bottom_left: Vector3::new(1.0, 0.0, 1.0),
                    texture: "game/stone".to_string(),
                    direction: 32,
                    edge: true,
                },
            ],
            colliders: vec![DeserialisedAabb {
                bottom_left: Vector3::new(0.0, 0.0, 0.0),
                size: Vector3::new(1.0, 1.0, 1.0),
                collidable: true,
            }],
            emission: [0, 0, 0, 0],
            loot_table: vec![DeserialisedLootTableEntry {
                chance: 1.0,
                item: "mcv3::StoneItem".to_string(),
            }],
        },
        DeserialisedBlock {
            identifier: "mcv3::Lamp".to_string(),
            translucent: true,
            full: false,
            draw_betweens: true,
            faces: vec![
                DeserialisedFace {
                    top_left: Vector3::new(0.25, 0.75, 0.25),
                    top_right: Vector3::new(0.75, 0.75, 0.25),
                    bottom_left: Vector3::new(0.25, 0.75, 0.75),
                    texture: "game/lamp_bottom".to_string(),
                    direction: 1,
                    edge: false,
                },
                DeserialisedFace {
                    top_left: Vector3::new(0.25, 0.0, 0.25),
                    top_right: Vector3::new(0.75, 0.0, 0.25),
                    bottom_left: Vector3::new(0.25, 0.0, 0.75),
                    texture: "game/lamp_bottom".to_string(),
                    direction: 2,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(0.25, 0.0, 0.25),
                    top_right: Vector3::new(0.25, 0.75, 0.25),
                    bottom_left: Vector3::new(0.25, 0.0, 0.75),
                    texture: "game/lamp_side".to_string(),
                    direction: 4,
                    edge: false,
                },
                DeserialisedFace {
                    top_left: Vector3::new(0.75, 0.0, 0.75),
                    top_right: Vector3::new(0.75, 0.75, 0.75),
                    bottom_left: Vector3::new(0.75, 0.0, 0.25),
                    texture: "game/lamp_side".to_string(),
                    direction: 8,
                    edge: false,
                },
                DeserialisedFace {
                    top_left: Vector3::new(0.75, 0.0, 0.25),
                    top_right: Vector3::new(0.75, 0.75, 0.25),
                    bottom_left: Vector3::new(0.25, 0.0, 0.25),
                    texture: "game/lamp_side".to_string(),
                    direction: 16,
                    edge: false,
                },
                DeserialisedFace {
                    top_left: Vector3::new(0.25, 0.0, 0.75),
                    top_right: Vector3::new(0.25, 0.75, 0.75),
                    bottom_left: Vector3::new(0.75, 0.0, 0.75),
                    texture: "game/lamp_side".to_string(),
                    direction: 32,
                    edge: false,
                },
            ],
            colliders: vec![DeserialisedAabb {
                bottom_left: Vector3::new(0.25, 0.0, 0.25),
                size: Vector3::new(0.5, 0.75, 0.5),
                collidable: true,
            }],
            emission: [200, 100, 100, 14],
            loot_table: vec![DeserialisedLootTableEntry {
                chance: 1.0,
                item: "mcv3::LampItem".to_string(),
            }],
        },
        DeserialisedBlock {
            identifier: "mcv3::Sand".to_string(),
            translucent: false,
            full: true,
            draw_betweens: false,
            faces: vec![
                DeserialisedFace {
                    top_left: Vector3::new(0.0, 1.0, 0.0),
                    top_right: Vector3::new(1.0, 1.0, 0.0),
                    bottom_left: Vector3::new(0.0, 1.0, 1.0),
                    texture: "game/sand".to_string(),
                    direction: 1,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(0.0, 0.0, 0.0),
                    top_right: Vector3::new(0.0, 0.0, 1.0),
                    bottom_left: Vector3::new(1.0, 0.0, 0.0),
                    texture: "game/sand".to_string(),
                    direction: 2,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(0.0, 0.0, 0.0),
                    top_right: Vector3::new(0.0, 1.0, 0.0),
                    bottom_left: Vector3::new(0.0, 0.0, 1.0),
                    texture: "game/sand".to_string(),
                    direction: 4,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(1.0, 0.0, 1.0),
                    top_right: Vector3::new(1.0, 1.0, 1.0),
                    bottom_left: Vector3::new(1.0, 0.0, 0.0),
                    texture: "game/sand".to_string(),
                    direction: 8,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(1.0, 0.0, 0.0),
                    top_right: Vector3::new(1.0, 1.0, 0.0),
                    bottom_left: Vector3::new(0.0, 0.0, 0.0),
                    texture: "game/sand".to_string(),
                    direction: 16,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(0.0, 0.0, 1.0),
                    top_right: Vector3::new(0.0, 1.0, 1.0),
                    bottom_left: Vector3::new(1.0, 0.0, 1.0),
                    texture: "game/sand".to_string(),
                    direction: 32,
                    edge: true,
                },
            ],
            colliders: vec![DeserialisedAabb {
                bottom_left: Vector3::new(0.0, 0.0, 0.0),
                size: Vector3::new(1.0, 1.0, 1.0),
                collidable: true,
            }],
            emission: [0, 0, 0, 0],
            loot_table: vec![],
        },
        DeserialisedBlock {
            identifier: "mcv3::ItemSpawnerBlock".to_string(),
            translucent: false,
            full: true,
            draw_betweens: false,
            faces: vec![
                DeserialisedFace {
                    top_left: Vector3::new(0.0, 1.0, 0.0),
                    top_right: Vector3::new(1.0, 1.0, 0.0),
                    bottom_left: Vector3::new(0.0, 1.0, 1.0),
                    texture: "game/itemspawner".to_string(),
                    direction: 1,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(0.0, 0.0, 0.0),
                    top_right: Vector3::new(0.0, 0.0, 1.0),
                    bottom_left: Vector3::new(1.0, 0.0, 0.0),
                    texture: "game/itemspawner".to_string(),
                    direction: 2,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(0.0, 0.0, 0.0),
                    top_right: Vector3::new(0.0, 1.0, 0.0),
                    bottom_left: Vector3::new(0.0, 0.0, 1.0),
                    texture: "game/itemspawner".to_string(),
                    direction: 4,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(1.0, 0.0, 1.0),
                    top_right: Vector3::new(1.0, 1.0, 1.0),
                    bottom_left: Vector3::new(1.0, 0.0, 0.0),
                    texture: "game/itemspawner".to_string(),
                    direction: 8,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(1.0, 0.0, 0.0),
                    top_right: Vector3::new(1.0, 1.0, 0.0),
                    bottom_left: Vector3::new(0.0, 0.0, 0.0),
                    texture: "game/itemspawner".to_string(),
                    direction: 16,
                    edge: true,
                },
                DeserialisedFace {
                    top_left: Vector3::new(0.0, 0.0, 1.0),
                    top_right: Vector3::new(0.0, 1.0, 1.0),
                    bottom_left: Vector3::new(1.0, 0.0, 1.0),
                    texture: "game/itemspawner".to_string(),
                    direction: 32,
                    edge: true,
                },
            ],
            colliders: vec![DeserialisedAabb {
                bottom_left: Vector3::new(0.0, 0.0, 0.0),
                size: Vector3::new(1.0, 1.0, 1.0),
                collidable: true,
            }],
            emission: [0, 0, 0, 0],
            loot_table: vec![DeserialisedLootTableEntry {
                chance: 1.0,
                item: "mcv3::ItemSpawnerItem".to_string(),
            }],
        },
    ];

    states.append(&mut pipe());

    fs::write(
        "./assets/game/state.blocks".to_string(),
        serde_json::to_string_pretty(&BlockStatesFile { states }).unwrap(),
    )
    .unwrap();
}
