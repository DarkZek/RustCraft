use crate::block::types::Block;
use crate::block::blocks::{BlockImpl, BlockUid};

pub struct AirBlock;

impl BlockImpl for AirBlock {
    const IDENTIFIER: &'static str = "mcv3::block::Air";

    fn get_variants() -> Vec<Block> {
        vec![
            Block {
                identifier: "mcv3::block::Air".to_string(),
                translucent: true,
                full: false,
                draw_betweens: false,
                faces: vec![],
                collision_boxes: vec![],
                bounding_boxes: vec![],
                emission: [0; 4],
            }
        ]
    }

    fn parse_block_state(id: BlockUid) -> Self {
        Self
    }
}