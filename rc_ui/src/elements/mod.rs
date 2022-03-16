pub mod button;
pub mod image;
pub mod rect;
pub mod text;

use crate::positioning::Layout;
use crate::vertex::UIVertex;
use nalgebra::Vector2;
use specs::World;

/// Describes an element on the screen, like a button, textfield or rectangle.
pub trait UIElement {
    /// Renders an element to UIVertexes
    fn render(&self, parent: &Layout) -> Vec<UIVertex>;

    /// Returns the offset and size respectively, used for hover handling
    fn position(&self) -> (Vector2<f32>, Vector2<f32>);

    /// Sets if an object is hovered
    fn hovered(&mut self, state: bool) -> bool {
        false
    }

    fn clicked(&mut self, universe: &World) {}
}

pub struct ElementData {
    pub data: Box<dyn UIElement + Sync + Send>,
    pub hovered: bool,
}

impl ElementData {
    pub fn wrap(data: Box<dyn UIElement + Sync + Send>) -> ElementData {
        ElementData {
            data,
            hovered: false,
        }
    }
}
