use crate::atlas::TextureAtlasIndex;
use crate::vertex::UIVertex;
use nalgebra::Vector2;

pub trait Lerp {
    fn lerp(self, b: Self, t: f32) -> Self;
}

impl Lerp for f32 {
    fn lerp(self, b: Self, t: f32) -> Self {
        ((b - self) * t) + self
    }
}

/// Draw rectangle
pub fn draw_rect(pos: Vector2<f32>, size: Vector2<f32>, color: [f32; 4]) -> Vec<UIVertex> {
    vec![
        UIVertex {
            position: [pos.x, pos.y],
            tex_coords: [-1.0, -1.0],
            color,
        },
        UIVertex {
            position: [pos.x + size.x, pos.y],
            tex_coords: [-1.0, -1.0],
            color,
        },
        UIVertex {
            position: [pos.x, pos.y + size.y],
            tex_coords: [-1.0, -1.0],
            color,
        },
        UIVertex {
            position: [pos.x + size.x, pos.y + size.y],
            tex_coords: [-1.0, -1.0],
            color,
        },
        UIVertex {
            position: [pos.x + size.x, pos.y],
            tex_coords: [-1.0, -1.0],
            color,
        },
        UIVertex {
            position: [pos.x, pos.y + size.y],
            tex_coords: [-1.0, -1.0],
            color,
        },
    ]
}

pub fn draw_sprite<'a>(
    pos: Vector2<f32>,
    size: Vector2<f32>,
    index: TextureAtlasIndex,
    color: [f32; 4],
) -> Vec<UIVertex> {
    vec![
        UIVertex {
            position: [pos.x, pos.y],
            tex_coords: [index.u_min, index.v_min],
            color,
        },
        UIVertex {
            position: [pos.x + size.x, pos.y],
            tex_coords: [index.u_max, index.v_min],
            color,
        },
        UIVertex {
            position: [pos.x, pos.y + size.y],
            tex_coords: [index.u_min, index.v_max],
            color,
        },
        UIVertex {
            position: [pos.x + size.x, pos.y + size.y],
            tex_coords: [index.u_max, index.v_max],
            color,
        },
        UIVertex {
            position: [pos.x + size.x, pos.y],
            tex_coords: [index.u_max, index.v_min],
            color,
        },
        UIVertex {
            position: [pos.x, pos.y + size.y],
            tex_coords: [index.u_min, index.v_max],
            color,
        },
    ]
}
