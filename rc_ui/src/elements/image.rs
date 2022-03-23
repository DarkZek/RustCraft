use crate::atlas::TextureAtlasIndex;
use crate::elements::UIElement;
use crate::helpers::draw_sprite;
use crate::positioning::Layout;
use crate::vertex::UIVertex;
use nalgebra::Vector2;

pub struct UIImage {
    layout: Layout,
    color: [f32; 4],
    index: TextureAtlasIndex,
}

impl UIImage {
    pub fn new(layout: Layout, index: TextureAtlasIndex) -> Box<UIImage> {
        Box::new(UIImage {
            layout,
            color: [1.0; 4],
            index,
        })
    }
}

impl UIElement for UIImage {
    fn render(&self, layout: &Layout) -> Vec<UIVertex> {
        let pos = self.layout.position_object(layout);
        draw_sprite(pos, self.layout.size, self.index, self.color)
    }

    fn position(&self) -> (Vector2<f32>, Vector2<f32>) {
        (self.layout.offset, self.layout.size)
    }
}
