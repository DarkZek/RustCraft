use crate::block::BlockId;
use crate::block::types::VisualBlock;
use crate::block::blocks::{BlockImpl};

pub struct AirBlock;

impl BlockImpl for AirBlock {
    const IDENTIFIER: &'static str = "mcv3::block::Air";

    fn get_variants() -> Vec<VisualBlock> {
        vec![
            VisualBlock {
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

    fn parse_block_state(id: BlockId) -> Self {
        Self
    }
}