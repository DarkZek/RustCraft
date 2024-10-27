use nalgebra::Vector3;
use crate::aabb::Aabb;
use crate::block::BlockId;
use crate::block::types::VisualBlock;
use crate::block::blocks::{BlockImpl, get_full_block_faces};

pub struct PlasterBlock;

impl BlockImpl for PlasterBlock {
    const IDENTIFIER: &'static str = "mcv3::block::Plaster";

    fn get_variants() -> Vec<VisualBlock> {
        vec![
            VisualBlock {
                translucent: false,
                full: true,
                draw_betweens: false,
                faces: get_full_block_faces("game/plaster"),
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

    fn parse_block_state(id: BlockId) -> Self {
        Self
    }
}