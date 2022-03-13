use crate::elements::image::UIImage;
use crate::elements::UIElement;
use crate::vertex::UIVertex;
use nalgebra::Vector2;

pub struct UIRect {
    pos: Vector2<f32>,
    size: Vector2<f32>,
    color: [f32; 4],
}

impl UIRect {
    pub fn new(pos: Vector2<f32>, size: Vector2<f32>, color: [f32; 4]) -> Box<UIRect> {
        Box::new(UIRect { pos, size, color })
    }
}

impl UIElement for UIRect {
    fn render(&self) -> Vec<UIVertex> {
        vec![
            UIVertex {
                position: [self.pos.x, self.pos.y],
                tex_coords: [-1.0, -1.0],
                color: self.color.clone(),
            },
            UIVertex {
                position: [self.pos.x + self.size.x, self.pos.y],
                tex_coords: [-1.0, -1.0],
                color: self.color.clone(),
            },
            UIVertex {
                position: [self.pos.x, self.pos.y + self.size.y],
                tex_coords: [-1.0, -1.0],
                color: self.color.clone(),
            },
            UIVertex {
                position: [self.pos.x + self.size.x, self.pos.y + self.size.y],
                tex_coords: [-1.0, -1.0],
                color: self.color.clone(),
            },
            UIVertex {
                position: [self.pos.x + self.size.x, self.pos.y],
                tex_coords: [-1.0, -1.0],
                color: self.color.clone(),
            },
            UIVertex {
                position: [self.pos.x, self.pos.y + self.size.y],
                tex_coords: [-1.0, -1.0],
                color: self.color.clone(),
            },
        ]
    }
}
