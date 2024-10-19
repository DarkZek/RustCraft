use crate::block::blocks::TextureAtlasIndex;
use nalgebra::Vector3;
use crate::aabb::Aabb;
use crate::atlas::TEXTURE_ATLAS;
use crate::block::face::Face;
use crate::block::types::Block;
use crate::block::blocks::{BlockImpl, BlockUid, get_full_block_faces};
use crate::viewable_direction::ViewableDirectionBitMap;

pub struct WaterBlock;

impl BlockImpl for WaterBlock {
    const IDENTIFIER: &'static str = "mcv3::block::Water";

    fn get_variants() -> Vec<Block> {
        let texture = *TEXTURE_ATLAS.get().index.get("game/water").unwrap_or(&TextureAtlasIndex::default());

        vec![
            Block {
                // TODO: Refactor code to use `BlockImpl::IDENTIFIER`
                identifier: "mcv3::block::Water".to_string(),
                translucent: true,
                full: false,
                draw_betweens: false,
                faces: vec![
                    Face {
                        top_left: Vector3::new(0.0, 0.9, 0.0),
                        top_right: Vector3::new(1.0, 0.9, 0.0),
                        bottom_left: Vector3::new(0.0, 0.9, 1.0),
                        edge: false,
                        direction: ViewableDirectionBitMap::Top,
                        wind_strengths: None,
                        normal: ViewableDirectionBitMap::Top.to_normal(),
                        texture
                    },
                    Face {
                        top_left: Vector3::new(0.0, 0.0, 0.0),
                        top_right: Vector3::new(0.0, 0.0, 1.0),
                        bottom_left: Vector3::new(1.0, 0.0, 0.0),
                        edge: false,
                        direction: ViewableDirectionBitMap::Bottom,
                        wind_strengths: None,
                        normal: ViewableDirectionBitMap::Bottom.to_normal(),
                        texture
                    },
                    Face {
                        top_left: Vector3::new(0.0, 0.0, 0.0),
                        top_right: Vector3::new(0.0, 0.9, 0.0),
                        bottom_left: Vector3::new(0.0, 0.0, 1.0),
                        edge: false,
                        direction: ViewableDirectionBitMap::Left,
                        wind_strengths: None,
                        normal: ViewableDirectionBitMap::Left.to_normal(),
                        texture
                    },
                    Face {
                        top_left: Vector3::new(1.0, 0.0, 1.0),
                        top_right: Vector3::new(1.0, 0.9, 1.0),
                        bottom_left: Vector3::new(1.0, 0.0, 0.0),
                        edge: false,
                        direction: ViewableDirectionBitMap::Right,
                        wind_strengths: None,
                        normal: ViewableDirectionBitMap::Right.to_normal(),
                        texture
                    },
                    Face {
                        top_left: Vector3::new(1.0, 0.0, 0.0),
                        top_right: Vector3::new(1.0, 0.9, 1.0),
                        bottom_left: Vector3::new(0.0, 0.0, 0.0),
                        edge: false,
                        direction: ViewableDirectionBitMap::Front,
                        wind_strengths: None,
                        normal: ViewableDirectionBitMap::Front.to_normal(),
                        texture
                    },
                    Face {
                        top_left: Vector3::new(0.0, 0.0, 1.0),
                        top_right: Vector3::new(0.0, 0.9, 1.0),
                        bottom_left: Vector3::new(1.0, 0.0, 1.0),
                        edge: false,
                        direction: ViewableDirectionBitMap::Back,
                        wind_strengths: None,
                        normal: ViewableDirectionBitMap::Back.to_normal(),
                        texture
                    },
                ],
                collision_boxes: vec![],
                bounding_boxes: vec![
                    Aabb::new(
                        Vector3::new(0.0, 0.0, 0.0),
                        Vector3::new(1.0, 0.9, 1.0),
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