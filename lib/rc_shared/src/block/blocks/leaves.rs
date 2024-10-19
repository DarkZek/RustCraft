use nalgebra::Vector3;
use crate::aabb::Aabb;
use crate::atlas::{TEXTURE_ATLAS, TextureAtlasIndex};
use crate::block::face::Face;
use crate::block::types::Block;
use crate::block::blocks::{BlockImpl, BlockUid, get_full_block_faces};
use crate::viewable_direction::ViewableDirectionBitMap;

pub struct LeavesBlock;

impl BlockImpl for LeavesBlock {
    const IDENTIFIER: &'static str = "mcv3::block::Leaves";

    fn get_variants() -> Vec<Block> {
        let texture = *TEXTURE_ATLAS.get().index.get("game/tree_leaves").unwrap_or(&TextureAtlasIndex::default());

        vec![
            Block {
                identifier: "mcv3::block::Leaves".to_string(),
                translucent: true,
                full: true,
                draw_betweens: true,
                faces: vec![
                    Face {
                        top_left: Vector3::new(0.0, 1.0, 0.0),
                        top_right: Vector3::new(1.0, 1.0, 0.0),
                        bottom_left: Vector3::new(0.0, 1.0, 1.0),
                        edge: true,
                        direction: ViewableDirectionBitMap::Top,
                        wind_strengths: Some([0.7, 0.7, 0.7, 0.7]),
                        normal: ViewableDirectionBitMap::Top.to_normal(),
                        texture
                    },
                    Face {
                        top_left: Vector3::new(0.0, 0.0, 0.0),
                        top_right: Vector3::new(0.0, 0.0, 1.0),
                        bottom_left: Vector3::new(1.0, 0.0, 0.0),
                        edge: true,
                        direction: ViewableDirectionBitMap::Bottom,
                        wind_strengths: Some([0.7, 0.7, 0.7, 0.7]),
                        normal: ViewableDirectionBitMap::Bottom.to_normal(),
                        texture
                    },
                    Face {
                        top_left: Vector3::new(0.0, 0.0, 0.0),
                        top_right: Vector3::new(0.0, 1.0, 0.0),
                        bottom_left: Vector3::new(0.0, 0.0, 1.0),
                        edge: true,
                        direction: ViewableDirectionBitMap::Left,
                        wind_strengths: Some([0.7, 0.7, 0.7, 0.7]),
                        normal: ViewableDirectionBitMap::Left.to_normal(),
                        texture
                    },
                    Face {
                        top_left: Vector3::new(1.0, 0.0, 1.0),
                        top_right: Vector3::new(1.0, 1.0, 1.0),
                        bottom_left: Vector3::new(1.0, 0.0, 0.0),
                        edge: true,
                        direction: ViewableDirectionBitMap::Right,
                        wind_strengths: Some([0.7, 0.7, 0.7, 0.7]),
                        normal: ViewableDirectionBitMap::Right.to_normal(),
                        texture
                    },
                    Face {
                        top_left: Vector3::new(1.0, 0.0, 0.0),
                        top_right: Vector3::new(1.0, 1.0, 0.0),
                        bottom_left: Vector3::new(0.0, 0.0, 0.0),
                        edge: true,
                        direction: ViewableDirectionBitMap::Front,
                        wind_strengths: Some([0.7, 0.7, 0.7, 0.7]),
                        normal: ViewableDirectionBitMap::Front.to_normal(),
                        texture
                    },
                    Face {
                        top_left: Vector3::new(0.0, 0.0, 1.0),
                        top_right: Vector3::new(0.0, 1.0, 1.0),
                        bottom_left: Vector3::new(1.0, 0.0, 1.0),
                        edge: true,
                        direction: ViewableDirectionBitMap::Back,
                        wind_strengths: Some([0.7, 0.7, 0.7, 0.7]),
                        normal: ViewableDirectionBitMap::Back.to_normal(),
                        texture
                    }
                ],
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