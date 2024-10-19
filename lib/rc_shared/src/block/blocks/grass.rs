use nalgebra::Vector3;
use crate::aabb::Aabb;
use crate::atlas::{TEXTURE_ATLAS, TextureAtlasIndex};
use crate::block::face::Face;
use crate::block::types::{Block, LootTableEntry};
use crate::block::blocks::{BlockImpl, BlockUid, get_full_block_faces};
use crate::viewable_direction::ViewableDirectionBitMap;

pub struct GrassBlock;

impl BlockImpl for GrassBlock {
    const IDENTIFIER: &'static str = "mcv3::block::Grass";

    fn get_variants() -> Vec<Block> {
        let texture_top = *TEXTURE_ATLAS.get().index.get("game/grass_top").unwrap_or(&TextureAtlasIndex::default());
        let texture_side = *TEXTURE_ATLAS.get().index.get("game/grass_side").unwrap_or(&TextureAtlasIndex::default());
        let texture_bottom = *TEXTURE_ATLAS.get().index.get("game/dirt").unwrap_or(&TextureAtlasIndex::default());

        vec![
            Block {
                // TODO: Refactor code to use `BlockImpl::IDENTIFIER`
                identifier: "mcv3::block::Grass".to_string(),
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
                        texture: texture_top
                    },
                    Face {
                        top_left: Vector3::new(0.0, 0.0, 0.0),
                        top_right: Vector3::new(0.0, 0.0, 1.0),
                        bottom_left: Vector3::new(1.0, 0.0, 0.0),
                        edge: true,
                        direction: ViewableDirectionBitMap::Bottom,
                        wind_strengths: None,
                        normal: ViewableDirectionBitMap::Bottom.to_normal(),
                        texture: texture_bottom
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

    fn parse_block_state(id: BlockUid) -> Self {
        Self
    }

    fn get_loot(&self) -> Vec<LootTableEntry> {
        vec![
            LootTableEntry {
                chance: 1.0,
                item_identifier: "mcv3::DirtBlockItem".to_string(),
            }
        ]
    }
}