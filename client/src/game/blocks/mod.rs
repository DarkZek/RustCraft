pub mod loader;
pub mod loading;
pub mod states;

use crate::game::blocks::states::BlockStates;
use crate::game::mesh::draw_kit::DrawKit;
use crate::game::mesh::face::Face;
use crate::game::viewable_direction::{ViewableDirection, ViewableDirectionBitMap};
use crate::services::asset::atlas::index::TextureAtlasIndex;

use crate::services::physics::aabb::Aabb;
use bevy::prelude::*;
use nalgebra::Vector3;
use std::collections::HashMap;

pub fn create_block_states(server: Res<AssetServer>, mut states: ResMut<BlockStates>) {
    states.asset = Some(server.load("game/block_states.blocks"));
}

#[derive(Debug, Clone)]
pub struct Block {
    pub identifier: String,
    pub translucent: bool,
    pub full: bool,
    pub draw_betweens: bool,
    pub faces: Vec<Face>,
    pub bounding_boxes: Vec<Aabb>,
}

impl Block {
    pub fn draw(&self, pos: Vector3<f32>, visible_map: ViewableDirection, mut kit: DrawKit) {
        for face in &self.faces {
            if !visible_map.has_flag(face.direction) && face.edge {
                // Not visible from that direction and marked as an edge face, so cull
                continue;
            }

            kit.draw_face(pos, face);
        }
    }
}

pub trait BlockGenerator {
    fn generate(texture_mapping: HashMap<String, TextureAtlasIndex>) -> Block;
}
