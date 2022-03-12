use nalgebra::Vector2;
use rc_ui::component::UIComponent;
use rc_ui::elements::image::UIImage;
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
                Vector2::new(0.0, 25.0),
                Vector2::new(50.0, 50.0),
                Vector2::new(0.0, 0.0),
                LayoutScheme::Center,
            ),
        }
    }
}

impl UIComponent for CrosshairComponent {
    fn render(&self) -> Vec<Box<dyn UIElement + 'static>> {
        vec![
            Box::new(UIRect::new(
                Vector2::new(48.0, 0.0),
                Vector2::new(4.0, 50.0),
                [255.0; 4],
            )),
            Box::new(UIRect::new(
                Vector2::new(0.0, 48.0),
                Vector2::new(50.0, 4.0),
                [255.0; 4],
            )),
        ]
    }

    fn rerender(&self) -> bool {
        true
    }

    fn positioning(&self) -> &Layout {
        &self.layout
    }
}
