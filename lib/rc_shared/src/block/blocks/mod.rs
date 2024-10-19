mod air;
mod dirt;
mod grass;
mod long_grass;
mod wood_log;
mod leaves;
mod stone;
mod lamp;
mod sand;
mod plaster;
mod water;
mod pipe;

use crate::block::blocks::stone::StoneBlock;
use nalgebra::Vector3;
use crate::atlas::{TEXTURE_ATLAS, TextureAtlasIndex};
use crate::block::face::Face;
use crate::block::types::{Block, LootTableEntry};
use crate::block::blocks::air::AirBlock;
use crate::block::blocks::dirt::DirtBlock;
use crate::block::blocks::grass::GrassBlock;
use crate::block::blocks::lamp::LampBlock;
use crate::block::blocks::leaves::LeavesBlock;
use crate::block::blocks::long_grass::LongGrassBlock;
use crate::block::blocks::pipe::PipeBlock;
use crate::block::blocks::plaster::PlasterBlock;
use crate::block::blocks::sand::SandBlock;
use crate::block::blocks::water::WaterBlock;
use crate::block::blocks::wood_log::WoodLogBlock;
use crate::block::uid::hash_uid;
use crate::viewable_direction::ViewableDirectionBitMap;

// Convert to const oncecell
pub fn get_blocks() -> Vec<BlockLookup> {
    vec![
        BlockLookup::from_block_impl::<AirBlock>(),
        BlockLookup::from_block_impl::<DirtBlock>(),
        BlockLookup::from_block_impl::<GrassBlock>(),
        BlockLookup::from_block_impl::<LongGrassBlock>(),
        BlockLookup::from_block_impl::<WoodLogBlock>(),
        BlockLookup::from_block_impl::<LeavesBlock>(),
        BlockLookup::from_block_impl::<StoneBlock>(),
        BlockLookup::from_block_impl::<LampBlock>(),
        BlockLookup::from_block_impl::<SandBlock>(),
        BlockLookup::from_block_impl::<PipeBlock>(),
        BlockLookup::from_block_impl::<PlasterBlock>(),
        BlockLookup::from_block_impl::<WaterBlock>(),
    ]
}

pub type BlockUid = u64;

pub struct BlockLookup {
    pub uid: BlockUid,
    pub identifier: &'static str,
    pub get_variants: fn() -> Vec<Block>,
    pub get_loot: fn(BlockUid) -> Vec<LootTableEntry>,
    pub on_destroy: fn(BlockUid),
}

impl BlockLookup {
    pub fn from_block_impl<T: BlockImpl>() -> Self {
        Self {
            uid: hash_uid(&T::IDENTIFIER),
            identifier: T::IDENTIFIER,
            get_variants: T::get_variants,
            on_destroy: |uid| T::parse_block_state(uid).on_destroy(),
            get_loot: |uid| T::parse_block_state(uid).get_loot()
        }
    }
}

pub trait BlockImpl {
    const IDENTIFIER: &'static str;
    fn get_variants() -> Vec<Block>;
    fn parse_block_state(id: BlockUid) -> Self;
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
