pub mod image;

use crate::vertex::UIVertex;

pub trait UIElement {
    fn render(&self) -> Vec<UIVertex>;
}
