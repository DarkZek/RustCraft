use crate::game::viewable_direction::{ViewableDirection, ViewableDirectionBitMap};
use crate::services::asset::atlas::index::TextureAtlasIndex;
use nalgebra::{Vector2, Vector3};

pub struct Face {
    pub top_left: Vector3<f32>,
    pub size: Vector2<f32>,
    pub texture: TextureAtlasIndex,
    pub normal: Vector3<f32>,
    // If face is at the edge of a face, and its direction is against a block where it could be fulled, then cull the face
    pub edge: bool,
    pub direction: ViewableDirectionBitMap,
}

impl Face {
    pub fn new(
        top_left: Vector3<f32>,
        size: Vector2<f32>,
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
            size,
            texture,
            normal,
            edge,
            direction,
        }
    }

    pub fn full_block(texture: TextureAtlasIndex) -> [Face; 6] {
        [
            // Bottom
            Face::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector2::new(1.0, 1.0),
                texture,
                ViewableDirectionBitMap::Bottom,
                true,
            ),
            // Top
            Face::new(
                Vector3::new(0.0, 1.0, 0.0),
                Vector2::new(1.0, 1.0),
                texture,
                ViewableDirectionBitMap::Top,
                true,
            ),
            // Front
            Face::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector2::new(1.0, 1.0),
                texture,
                ViewableDirectionBitMap::Front,
                true,
            ),
            // Back
            Face::new(
                Vector3::new(0.0, 0.0, 1.0),
                Vector2::new(1.0, 1.0),
                texture,
                ViewableDirectionBitMap::Back,
                true,
            ),
            // Left
            Face::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector2::new(1.0, 1.0),
                texture,
                ViewableDirectionBitMap::Left,
                true,
            ),
            // Right
            Face::new(
                Vector3::new(1.0, 0.0, 0.0),
                Vector2::new(1.0, 1.0),
                texture,
                ViewableDirectionBitMap::Right,
                true,
            ),
        ]
    }
}
