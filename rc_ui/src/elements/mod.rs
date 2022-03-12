pub mod image;
pub mod rect;

use crate::vertex::UIVertex;

pub trait UIElement {
    fn render(&self) -> Vec<UIVertex>;
}
