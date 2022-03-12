use crate::elements::UIElement;
use crate::vertex::UIVertex;

pub struct UIImage {}

impl UIImage {
    pub fn new() {}
}

impl UIElement for UIImage {
    fn render(&self) -> Vec<UIVertex> {
        vec![
            UIVertex {
                position: [-1.0, -1.0],
                tex_coords: [0.0, 0.0],
                color: [1.0; 4],
            },
            UIVertex {
                position: [1.0, -1.0],
                tex_coords: [0.0, 0.0],
                color: [1.0; 4],
            },
            UIVertex {
                position: [1.0, 1.0],
                tex_coords: [0.0, 0.0],
                color: [1.0; 4],
            },
        ]
    }
}
