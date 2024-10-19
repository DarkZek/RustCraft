use nalgebra::Vector3;
use crate::aabb::Aabb;
use crate::block::types::{Block, LootTableEntry};
use crate::block::blocks::{BlockImpl, BlockUid, get_full_block_faces};

pub struct StoneBlock;

impl BlockImpl for StoneBlock {
    const IDENTIFIER: &'static str = "mcv3::block::Stone";

    fn get_variants() -> Vec<Block> {
        vec![
            Block {
                identifier: "mcv3::block::Stone".to_string(),
                translucent: false,
                full: true,
                draw_betweens: false,
                faces: get_full_block_faces("game/stone"),
                collision_boxes: vec![
                    Aabb::new(
                        Vector3::new(0.0, 0.0, 0.0),
                        Vector3::new(1.0, 1.0, 1.0),
                    )
                ],
                bounding_boxes: vec![
                    Aabb::new(
                        Vector3::new(0.0, 0.0, 0.0),
                        Vector3::new(1.0, 1.0, 1.0),
                    )
                ],
                emission: [0; 4],
            }
        ]
    }

    fn parse_block_state(id: BlockUid) -> Self {
        Self
    }

    fn get_loot(&self) -> Vec<LootTableEntry> {
        vec![
            LootTableEntry {
                chance: 1.0,
                item_identifier: "mcv3::StoneItem".to_string(),
            }
        ]
    }
}