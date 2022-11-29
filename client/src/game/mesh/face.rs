use crate::game::viewable_direction::ViewableDirectionBitMap;
use crate::services::asset::atlas::index::TextureAtlasIndex;
use nalgebra::Vector3;

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
