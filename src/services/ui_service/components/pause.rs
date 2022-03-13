use nalgebra::Vector2;
use rc_ui::atlas::TextureAtlasIndex;
use rc_ui::component::UIComponent;
use rc_ui::elements::image::UIImage;
use rc_ui::elements::rect::UIRect;
use rc_ui::elements::text::UIText;
use rc_ui::elements::UIElement;
use rc_ui::fonts::TextAlignment;
use rc_ui::positioning::{Layout, LayoutScheme};

pub struct PauseMenuComponent {
    layout: Layout,
}

impl PauseMenuComponent {
    pub fn new() -> PauseMenuComponent {
        // Texture is 362x42, starting at 1,1
        PauseMenuComponent {
            layout: Layout::new(
                Vector2::new(800.0, 600.0),
                Vector2::new(0.0, 0.0),
                LayoutScheme::Center,
                0.0,
            ),
        }
    }
}

impl UIComponent for PauseMenuComponent {
    fn render(&self) -> Vec<Box<dyn UIElement + 'static>> {
        vec![Box::new(UIText {
            text: "Test text!".to_string(),
            font_size: 20.0,
            color: [1.0; 4],
            layout: Layout::new(
                Vector2::new(500.0, 50.0),
                Vector2::new(50.0, 50.0),
                LayoutScheme::Center,
                0.0,
            ),
            alignment: TextAlignment::Left,
            background_color: [0.0; 4],
        })]
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
