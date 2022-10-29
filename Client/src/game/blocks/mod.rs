pub mod air;
pub mod dirt;
pub mod grass;
pub mod leaves;
pub mod wood;

use crate::game::blocks::air::generate_air_block;
use crate::game::blocks::dirt::generate_dirt_block;
use crate::game::blocks::grass::generate_grass_block;
use crate::game::blocks::leaves::generate_leaves_block;
use crate::game::blocks::wood::generate_wood_block;
use crate::game::mesh::draw_kit::DrawKit;
use crate::game::mesh::face::Face;
use crate::game::viewable_direction::{ViewableDirection, ViewableDirectionBitMap};
use nalgebra::{MatrixMN, Vector3};

pub struct Block {
    pub name: String,
    pub identifier: String,
    pub translucent: bool,
    pub full: bool,
    pub draw_betweens: bool,
    pub faces: Vec<Face>,
}

impl Block {
    pub fn draw(&self, pos: Vector3<f32>, visible_map: ViewableDirection, mut kit: DrawKit) {
        for face in &self.faces {
            if !visible_map.has_flag(face.direction) && face.edge {
                // Not visible from that direction and marked as an edge face, so cull
                continue;
            }
            // Draw based on direction to get winding order of vertices correct
            match face.direction {
                ViewableDirectionBitMap::Top => {
                    kit.draw_top_face(pos + face.top_left, face.size, face.texture)
                }
                ViewableDirectionBitMap::Bottom => {
                    kit.draw_bottom_face(pos + face.top_left, face.size, face.texture)
                }
                ViewableDirectionBitMap::Left => {
                    kit.draw_left_face(pos + face.top_left, face.size, face.texture)
                }
                ViewableDirectionBitMap::Right => {
                    kit.draw_right_face(pos + face.top_left, face.size, face.texture)
                }
                ViewableDirectionBitMap::Front => {
                    kit.draw_front_face(pos + face.top_left, face.size, face.texture)
                }
                ViewableDirectionBitMap::Back => {
                    kit.draw_back_face(pos + face.top_left, face.size, face.texture)
                }
            }
        }
    }
}

pub struct BlockStates {
    pub states: Vec<Block>,
}

impl BlockStates {
    pub fn new() -> BlockStates {
        BlockStates {
            states: vec![
                generate_air_block(),
                generate_dirt_block(),
                generate_grass_block(),
                generate_leaves_block(),
                generate_wood_block(),
            ],
        }
    }

    // Possibly remove, keeping it because it was in old version and I might need it
    pub fn get_block(&self, i: usize) -> &Block {
        // TODO: Return error block if out of range
        self.states.get(i).unwrap()
    }
}
