


use nalgebra::Vector2;

use rc_ui::component::UIComponent;
use rc_ui::elements::button::UIButton;



use rc_ui::elements::UIElement;

use rc_ui::positioning::{Layout, LayoutScheme};


pub struct PauseMenuComponent {
    layout: Layout,
    pub visible: bool,
}

impl PauseMenuComponent {
    pub fn new() -> PauseMenuComponent {
        PauseMenuComponent {
            layout: Layout::new(
                Vector2::new(600.0, 600.0),
                Vector2::new(0.0, 0.0),
                LayoutScheme::Center,
                0.0,
            ),
            visible: false,
        }
    }
}

impl UIComponent for PauseMenuComponent {
    fn render(&self) -> Vec<Box<dyn UIElement + Send + Sync + 'static>> {
        vec![UIButton::new(
            Layout {
                size: Vector2::new(600.0, 60.0),
                offset: Vector2::new(0.0, 0.0),
                scheme: LayoutScheme::Top,
                padding: 0.0,
            },
            String::from("Back To Game"),
        )]
    }

    fn rerender(&self) -> bool {
        false
    }

    fn positioning(&self) -> &Layout {
        &self.layout
    }

    fn resized(&mut self) {}

    fn visible(&self) -> bool {
        self.visible
    }
}
