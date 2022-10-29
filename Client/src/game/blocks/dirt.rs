use crate::game::blocks::{Block, BlockGenerator};
use crate::game::mesh::draw_kit::DrawKit;
use crate::game::viewable_direction::{ViewableDirection, ViewableDirectionBitMap};
use crate::services::asset::atlas::atlas::ATLAS_WIDTH;
use crate::services::asset::atlas::index::TextureAtlasIndex;
use crate::Vec3;
use nalgebra::{Vector2, Vector3};

pub static DIRT_BLOCK_GENERATOR: DirtGenerator = DirtGenerator;

pub struct DirtGenerator;

pub struct Dirt {
    grass: bool,
}

impl<'a> BlockGenerator<'a> for DirtGenerator {
    fn indexes(&self) -> usize {
        2
    }

    fn create(&self, index: usize) -> Box<dyn Block> {
        Box::new(Dirt { grass: index == 1 })
    }
}

impl Block for Dirt {
    fn name(&self) -> &'static str {
        "Dirt"
    }

    fn identifier(&self) -> &'static str {
        "dirt"
    }

    fn is_translucent(&self) -> bool {
        false
    }

    fn is_full(&self) -> bool {
        true
    }

    fn draw_betweens(&self) -> bool {
        true
    }

    fn draw(&self, position: Vector3<f32>, visibility: ViewableDirection, mut draw: DrawKit) {
        let i = 16.0 / ATLAS_WIDTH as f32;
        let dirt = TextureAtlasIndex {
            u_min: 0.0,
            u_max: i,
            v_min: 0.0,
            v_max: i,
        };
        let grass_top = TextureAtlasIndex {
            u_min: i,
            u_max: i * 2.0,
            v_min: i,
            v_max: i * 2.0,
        };
        let grass = TextureAtlasIndex {
            u_min: i,
            u_max: i * 2.0,
            v_min: 0.0,
            v_max: i,
        };
        if self.grass {
            draw.draw_full_block_textures(
                position,
                visibility,
                [dirt, grass_top, grass, grass, grass, grass],
            );
        } else {
            draw.draw_full_block(position, visibility, dirt);
        }
    }
}
