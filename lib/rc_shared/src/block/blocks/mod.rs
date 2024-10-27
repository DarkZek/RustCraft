pub(crate) mod air;
pub(crate) mod dirt;
pub(crate) mod grass;
pub(crate) mod long_grass;
pub(crate) mod wood_log;
pub(crate) mod leaves;
pub(crate) mod stone;
pub(crate) mod lamp;
pub(crate) mod sand;
pub(crate) mod plaster;
pub(crate) mod water;
pub(crate) mod pipe;

use nalgebra::Vector3;
use crate::atlas::{TEXTURE_ATLAS, TextureAtlasIndex};
use crate::block::BlockId;
use crate::block::face::Face;
use crate::block::types::{VisualBlock, LootTableEntry};
use crate::viewable_direction::ViewableDirectionBitMap;

pub trait BlockImpl {
    const IDENTIFIER: &'static str;
    fn get_variants() -> Vec<VisualBlock>;
    fn parse_block_state(id: BlockId) -> Self;
    fn draw(&self) -> VisualBlock { Self::get_variants().pop().unwrap() }
    fn on_destroy(&self) {}
    fn get_loot(&self) -> Vec<LootTableEntry> { vec![] }
}

fn get_full_block_faces(
    texture: &str
) -> Vec<Face> {
    let texture = *TEXTURE_ATLAS.get().index.get(texture).unwrap_or(&TextureAtlasIndex::default());
    vec![
        Face {
            top_left: Vector3::new(0.0, 1.0, 0.0),
            top_right: Vector3::new(1.0, 1.0, 0.0),
            bottom_left: Vector3::new(0.0, 1.0, 1.0),
            edge: true,
            direction: ViewableDirectionBitMap::Top,
            wind_strengths: None,
            normal: ViewableDirectionBitMap::Top.to_normal(),
            texture
        },
        Face {
            top_left: Vector3::new(0.0, 0.0, 0.0),
            top_right: Vector3::new(0.0, 0.0, 1.0),
            bottom_left: Vector3::new(1.0, 0.0, 0.0),
            edge: true,
            direction: ViewableDirectionBitMap::Bottom,
            wind_strengths: None,
            normal: ViewableDirectionBitMap::Bottom.to_normal(),
            texture
        },
        Face {
            top_left: Vector3::new(0.0, 0.0, 0.0),
            top_right: Vector3::new(0.0, 1.0, 0.0),
            bottom_left: Vector3::new(0.0, 0.0, 1.0),
            edge: true,
            direction: ViewableDirectionBitMap::Left,
            wind_strengths: None,
            normal: ViewableDirectionBitMap::Left.to_normal(),
            texture
        },
        Face {
            top_left: Vector3::new(1.0, 0.0, 1.0),
            top_right: Vector3::new(1.0, 1.0, 1.0),
            bottom_left: Vector3::new(1.0, 0.0, 0.0),
            edge: true,
            direction: ViewableDirectionBitMap::Right,
            wind_strengths: None,
            normal: ViewableDirectionBitMap::Right.to_normal(),
            texture
        },
        Face {
            top_left: Vector3::new(1.0, 0.0, 0.0),
            top_right: Vector3::new(1.0, 1.0, 0.0),
            bottom_left: Vector3::new(0.0, 0.0, 0.0),
            edge: true,
            direction: ViewableDirectionBitMap::Front,
            wind_strengths: None,
            normal: ViewableDirectionBitMap::Front.to_normal(),
            texture
        },
        Face {
            top_left: Vector3::new(0.0, 0.0, 1.0),
            top_right: Vector3::new(0.0, 1.0, 1.0),
            bottom_left: Vector3::new(1.0, 0.0, 1.0),
            edge: true,
            direction: ViewableDirectionBitMap::Back,
            wind_strengths: None,
            normal: ViewableDirectionBitMap::Back.to_normal(),
            texture
        }
    ]
}
