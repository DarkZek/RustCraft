use crate::atlas::TextureAtlasIndex;
use crate::elements::UIElement;
use crate::vertex::UIVertex;
use nalgebra::Vector2;

pub struct UIImage {
    pos: Vector2<f32>,
    size: Vector2<f32>,
    color: [f32; 4],
    index: TextureAtlasIndex,
}

impl UIImage {
    pub fn new(pos: Vector2<f32>, size: Vector2<f32>, index: TextureAtlasIndex) -> Box<UIImage> {
        Box::new(UIImage {
            pos,
            size,
            color: [1.0; 4],
            index,
        })
    }
}

impl UIElement for UIImage {
    fn render(&self) -> Vec<UIVertex> {
        vec![
            UIVertex {
                position: [self.pos.x, self.pos.y],
                tex_coords: [self.index.u_min, self.index.v_min],
                color: self.color.clone(),
            },
            UIVertex {
                position: [self.pos.x + self.size.x, self.pos.y],
                tex_coords: [self.index.u_max, self.index.v_min],
                color: self.color.clone(),
            },
            UIVertex {
                position: [self.pos.x, self.pos.y + self.size.y],
                tex_coords: [self.index.u_min, self.index.v_max],
                color: self.color.clone(),
            },
            UIVertex {
                position: [self.pos.x + self.size.x, self.pos.y + self.size.y],
                tex_coords: [self.index.u_max, self.index.v_max],
                color: self.color.clone(),
            },
            UIVertex {
                position: [self.pos.x + self.size.x, self.pos.y],
                tex_coords: [self.index.u_max, self.index.v_min],
                color: self.color.clone(),
            },
            UIVertex {
                position: [self.pos.x, self.pos.y + self.size.y],
                tex_coords: [self.index.u_min, self.index.v_max],
                color: self.color.clone(),
            },
        ]
    }
}
