use crate::atlas::TextureAtlasIndex;
use crate::elements::UIElement;
use crate::helpers::draw_sprite;
use crate::positioning::Layout;
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
    fn render(&self, layout: &Layout) -> Vec<UIVertex> {
        draw_sprite(self.pos, self.size, self.index, self.color)
    }

    fn position(&self) -> (Vector2<f32>, Vector2<f32>) {
        (self.pos, self.size)
    }
}
