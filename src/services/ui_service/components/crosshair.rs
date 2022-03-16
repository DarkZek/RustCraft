use nalgebra::Vector2;
use rc_ui::component::UIComponent;

use rc_ui::elements::rect::UIRect;
use rc_ui::elements::UIElement;
use rc_ui::positioning::{Layout, LayoutScheme};

pub struct CrosshairComponent {
    layout: Layout,
}

impl CrosshairComponent {
    pub fn new() -> CrosshairComponent {
        CrosshairComponent {
            layout: Layout::new(
                Vector2::new(20.0, 20.0),
                Vector2::new(0.0, 0.0),
                LayoutScheme::Center,
                0.0,
            ),
        }
    }
}

impl UIComponent for CrosshairComponent {
    fn render(&self) -> Vec<Box<dyn UIElement + Send + Sync + 'static>> {
        vec![
            UIRect::new(Vector2::new(8.0, 0.0), Vector2::new(4.0, 20.0), [1.0; 4]),
            UIRect::new(Vector2::new(0.0, 8.0), Vector2::new(20.0, 4.0), [1.0; 4]),
        ]
    }

    fn rerender(&self) -> bool {
        false
    }

    fn positioning(&self) -> &Layout {
        &self.layout
    }

    fn resized(&mut self) {}

    fn visible(&self) -> bool {
        true
    }
}
