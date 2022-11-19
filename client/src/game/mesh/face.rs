use crate::game::viewable_direction::ViewableDirectionBitMap;
use crate::services::asset::atlas::index::TextureAtlasIndex;
use nalgebra::{Vector3};

#[derive(Debug, Clone)]
pub struct Face {
    pub top_left: Vector3<f32>,
    pub top_right: Vector3<f32>,
    pub bottom_left: Vector3<f32>,
    pub texture: TextureAtlasIndex,
    pub normal: Vector3<f32>,
    // If face is at the edge of a face, and its direction is against a block where it could be fulled, then cull the face
    pub edge: bool,
    pub direction: ViewableDirectionBitMap,
}

impl Face {
    pub fn new(
        top_left: Vector3<f32>,
        top_right: Vector3<f32>,
        bottom_left: Vector3<f32>,
        texture: TextureAtlasIndex,
        direction: ViewableDirectionBitMap,
        edge: bool,
    ) -> Face {
        let normal = match direction {
            ViewableDirectionBitMap::Top => Vector3::new(0.0, 1.0, 0.0),
            ViewableDirectionBitMap::Bottom => Vector3::new(0.0, -1.0, 0.0),
            ViewableDirectionBitMap::Left => Vector3::new(0.0, 0.0, 1.0),
            ViewableDirectionBitMap::Right => Vector3::new(0.0, 0.0, -1.0),
            ViewableDirectionBitMap::Front => Vector3::new(1.0, 0.0, 0.0),
            ViewableDirectionBitMap::Back => Vector3::new(-1.0, 0.0, 0.0),
        };
        Face {
            top_left,
            top_right,
            bottom_left,
            texture,
            normal,
            edge,
            direction,
        }
    }
}
