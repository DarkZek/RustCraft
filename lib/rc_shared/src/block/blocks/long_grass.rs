use nalgebra::Vector3;
use crate::aabb::Aabb;
use crate::atlas::{TEXTURE_ATLAS, TextureAtlasIndex};
use crate::block::BlockId;
use crate::block::face::Face;
use crate::block::types::VisualBlock;
use crate::block::blocks::{BlockImpl};
use crate::viewable_direction::ViewableDirectionBitMap;

pub struct LongGrassBlock;

impl BlockImpl for LongGrassBlock {
    const IDENTIFIER: &'static str = "mcv3::block::LongGrass";

    fn get_variants() -> Vec<VisualBlock> {
        let texture = *TEXTURE_ATLAS.get().index.get("game/long_grass").unwrap_or(&TextureAtlasIndex::default());
        vec![
            VisualBlock {
                // TODO: Refactor code to use `BlockImpl::IDENTIFIER`
                translucent: true,
                full: false,
                draw_betweens: true,
                faces: vec![
                    Face {
                        top_left: Vector3::new(0.0, 0.0, 0.0),
                        top_right: Vector3::new(0.0, 1.0, 0.0),
                        bottom_left: Vector3::new(1.0, 0.0, 1.0),
                        edge: false,
                        // TODO: Fix this
                        direction: ViewableDirectionBitMap::Top,
                        wind_strengths: Some([0.0, 1.0, 1.0, 0.0]),
                        normal: ViewableDirectionBitMap::Top.to_normal(),
                        texture
                    },
                    Face {
                        top_left: Vector3::new(1.0, 0.0, 1.0),
                        top_right: Vector3::new(1.0, 1.0, 1.0),
                        bottom_left: Vector3::new(0.0, 0.0, 0.0),
                        edge: false,
                        // TODO: Fix this
                        direction: ViewableDirectionBitMap::Top,
                        wind_strengths: Some([0.0, 1.0, 1.0, 0.0]),
                        normal: ViewableDirectionBitMap::Top.to_normal(),
                        texture
                    },
                    Face {
                        top_left: Vector3::new(0.0, 0.0, 1.0),
                        top_right: Vector3::new(0.0, 1.0, 1.0),
                        bottom_left: Vector3::new(1.0, 0.0, 0.0),
                        edge: false,
                        // TODO: Fix this
                        direction: ViewableDirectionBitMap::Top,
                        wind_strengths: Some([0.0, 1.0, 1.0, 0.0]),
                        normal: ViewableDirectionBitMap::Top.to_normal(),
                        texture
                    },
                    Face {
                        top_left: Vector3::new(1.0, 0.0, 0.0),
                        top_right: Vector3::new(1.0, 1.0, 0.0),
                        bottom_left: Vector3::new(0.0, 0.0, 1.0),
                        edge: false,
                        // TODO: Fix this
                        direction: ViewableDirectionBitMap::Top,
                        wind_strengths: Some([0.0, 1.0, 1.0, 0.0]),
                        normal: ViewableDirectionBitMap::Top.to_normal(),
                        texture
                    }
                ],
                collision_boxes: vec![],
                bounding_boxes: vec![
                    Aabb::new(
                        Vector3::new(0.2, 0.0, 0.2),
                        Vector3::new(0.6, 0.8, 0.6),
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