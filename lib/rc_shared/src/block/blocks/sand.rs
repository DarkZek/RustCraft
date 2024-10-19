use nalgebra::Vector3;
use crate::aabb::Aabb;
use crate::block::types::Block;
use crate::block::blocks::{BlockImpl, BlockUid, get_full_block_faces};

pub struct SandBlock;

impl BlockImpl for SandBlock {
    const IDENTIFIER: &'static str = "mcv3::block::Sand";

    fn get_variants() -> Vec<Block> {
        vec![
            Block {
                identifier: "mcv3::block::Sand".to_string(),
                translucent: false,
                full: true,
                draw_betweens: false,
                faces: get_full_block_faces("game/sand"),
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
}