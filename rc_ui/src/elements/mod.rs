pub mod image;
pub mod rect;
pub mod text;

use crate::positioning::Layout;
use crate::vertex::UIVertex;

pub trait UIElement {
    fn render(&self, parent: &Layout) -> Vec<UIVertex>;
}
