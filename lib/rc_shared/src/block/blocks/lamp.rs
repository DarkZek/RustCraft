use crate::block::blocks::TextureAtlasIndex;
use nalgebra::Vector3;
use crate::aabb::Aabb;
use crate::atlas::TEXTURE_ATLAS;
use crate::block::BlockId;
use crate::block::face::Face;
use crate::block::types::VisualBlock;
use crate::block::blocks::{BlockImpl, get_full_block_faces};
use crate::viewable_direction::ViewableDirectionBitMap;

pub struct LampBlock;

impl BlockImpl for LampBlock {
    const IDENTIFIER: &'static str = "mcv3::block::Lamp";

    fn get_variants() -> Vec<VisualBlock> {
        let texture_y = *TEXTURE_ATLAS.get().index.get("game/lamp_bottom").unwrap_or(&TextureAtlasIndex::default());
        let texture_side = *TEXTURE_ATLAS.get().index.get("game/lamp_side").unwrap_or(&TextureAtlasIndex::default());

        vec![
            VisualBlock {
                translucent: true,
                full: false,
                draw_betweens: true,
                faces: vec![
                    Face {
                        top_left: Vector3::new(0.25, 0.75, 0.25),
                        top_right: Vector3::new(0.75, 0.75, 0.25),
                        bottom_left: Vector3::new(0.25, 0.75, 0.75),
                        edge: false,
                        direction: ViewableDirectionBitMap::Top,
                        wind_strengths: None,
                        normal: ViewableDirectionBitMap::Top.to_normal(),
                        texture: texture_y
                    },
                    Face {
                        top_left: Vector3::new(0.75, 0.0, 0.25),
                        top_right: Vector3::new(0.25, 0.0, 0.25),
                        bottom_left: Vector3::new(0.75, 0.0, 0.75),
                        edge: true,
                        direction: ViewableDirectionBitMap::Bottom,
                        wind_strengths: None,
                        normal: ViewableDirectionBitMap::Bottom.to_normal(),
                        texture: texture_y
                    },
                    Face {
                        top_left: Vector3::new(0.25, 0.0, 0.25),
                        top_right: Vector3::new(0.25, 0.75, 0.25),
                        bottom_left: Vector3::new(0.25, 0.0, 0.75),
                        edge: true,
                        direction: ViewableDirectionBitMap::Left,
                        wind_strengths: None,
                        normal: ViewableDirectionBitMap::Left.to_normal(),
                        texture: texture_side
                    },
                    Face {
                        top_left: Vector3::new(0.75, 0.0, 0.75),
                        top_right: Vector3::new(0.75, 0.75, 0.75),
                        bottom_left: Vector3::new(0.75, 0.0, 0.25),
                        edge: true,
                        direction: ViewableDirectionBitMap::Right,
                        wind_strengths: None,
                        normal: ViewableDirectionBitMap::Right.to_normal(),
                        texture: texture_side
                    },
                    Face {
                        top_left: Vector3::new(0.75, 0.0, 0.25),
                        top_right: Vector3::new(0.75, 0.75, 0.25),
                        bottom_left: Vector3::new(0.25, 0.0, 0.25),
                        edge: true,
                        direction: ViewableDirectionBitMap::Front,
                        wind_strengths: None,
                        normal: ViewableDirectionBitMap::Front.to_normal(),
                        texture: texture_side
                    },
                    Face {
                        top_left: Vector3::new(0.25, 0.0, 0.75),
                        top_right: Vector3::new(0.25, 0.75, 0.75),
                        bottom_left: Vector3::new(0.75, 0.0, 0.75),
                        edge: true,
                        direction: ViewableDirectionBitMap::Back,
                        wind_strengths: None,
                        normal: ViewableDirectionBitMap::Back.to_normal(),
                        texture: texture_side
                    }
                ],
                collision_boxes: vec![
                    Aabb::new(
                        Vector3::new(0.25, 0.0, 0.25),
                        Vector3::new(0.5, 0.75, 0.5),
                    )
                ],
                bounding_boxes: vec![
                    Aabb::new(
                        Vector3::new(0.25, 0.0, 0.25),
                        Vector3::new(0.5, 0.75, 0.5),
                    )
                ],
                emission: [255, 180, 80, 16],
            }
        ]
    }

    fn parse_block_state(id: BlockId) -> Self {
        Self
    }
}