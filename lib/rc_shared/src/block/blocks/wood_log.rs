use nalgebra::Vector3;
use crate::aabb::Aabb;
use crate::atlas::{TEXTURE_ATLAS, TextureAtlasIndex};
use crate::block::BlockId;
use crate::block::face::Face;
use crate::block::types::{VisualBlock, LootTableEntry};
use crate::block::blocks::{BlockImpl, get_full_block_faces};
use crate::viewable_direction::ViewableDirectionBitMap;

pub struct WoodLogBlock;

impl BlockImpl for WoodLogBlock {
    const IDENTIFIER: &'static str = "mcv3::block::WoodLog";

    fn get_variants() -> Vec<VisualBlock> {
        let texture_y = *TEXTURE_ATLAS.get().index.get("game/wood_top").unwrap_or(&TextureAtlasIndex::default());
        let texture_side = *TEXTURE_ATLAS.get().index.get("game/wood_log").unwrap_or(&TextureAtlasIndex::default());

        vec![
            VisualBlock {
                // TODO: Refactor code to use `BlockImpl::IDENTIFIER`
                translucent: false,
                full: true,
                draw_betweens: false,
                faces: vec![
                    Face {
                        top_left: Vector3::new(0.0, 1.0, 0.0),
                        top_right: Vector3::new(1.0, 1.0, 0.0),
                        bottom_left: Vector3::new(0.0, 1.0, 1.0),
                        edge: true,
                        direction: ViewableDirectionBitMap::Top,
                        wind_strengths: None,
                        normal: ViewableDirectionBitMap::Top.to_normal(),
                        texture: texture_y
                    },
                    Face {
                        top_left: Vector3::new(0.0, 0.0, 0.0),
                        top_right: Vector3::new(0.0, 0.0, 1.0),
                        bottom_left: Vector3::new(1.0, 0.0, 0.0),
                        edge: true,
                        direction: ViewableDirectionBitMap::Bottom,
                        wind_strengths: None,
                        normal: ViewableDirectionBitMap::Bottom.to_normal(),
                        texture: texture_y
                    },
                    Face {
                        top_left: Vector3::new(0.0, 0.0, 0.0),
                        top_right: Vector3::new(0.0, 1.0, 0.0),
                        bottom_left: Vector3::new(0.0, 0.0, 1.0),
                        edge: true,
                        direction: ViewableDirectionBitMap::Left,
                        wind_strengths: None,
                        normal: ViewableDirectionBitMap::Left.to_normal(),
                        texture: texture_side
                    },
                    Face {
                        top_left: Vector3::new(1.0, 0.0, 1.0),
                        top_right: Vector3::new(1.0, 1.0, 1.0),
                        bottom_left: Vector3::new(1.0, 0.0, 0.0),
                        edge: true,
                        direction: ViewableDirectionBitMap::Right,
                        wind_strengths: None,
                        normal: ViewableDirectionBitMap::Right.to_normal(),
                        texture: texture_side
                    },
                    Face {
                        top_left: Vector3::new(1.0, 0.0, 0.0),
                        top_right: Vector3::new(1.0, 1.0, 0.0),
                        bottom_left: Vector3::new(0.0, 0.0, 0.0),
                        edge: true,
                        direction: ViewableDirectionBitMap::Front,
                        wind_strengths: None,
                        normal: ViewableDirectionBitMap::Front.to_normal(),
                        texture: texture_side
                    },
                    Face {
                        top_left: Vector3::new(0.0, 0.0, 1.0),
                        top_right: Vector3::new(0.0, 1.0, 1.0),
                        bottom_left: Vector3::new(1.0, 0.0, 1.0),
                        edge: true,
                        direction: ViewableDirectionBitMap::Back,
                        wind_strengths: None,
                        normal: ViewableDirectionBitMap::Back.to_normal(),
                        texture: texture_side
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

    fn parse_block_state(id: BlockId) -> Self {
        Self
    }

    fn get_loot(&self) -> Vec<LootTableEntry> {
        vec![
            LootTableEntry {
                chance: 1.0,
                item_identifier: "mcv3::WoodLogItem".to_string(),
            }
        ]
    }
}