use crate::render::RenderState;
use crate::services::input_service::actions::ActionSheet;
use crate::services::input_service::input::InputState;
use crate::services::ui_service::components::UIComponents;
use crate::services::ui_service::UIService;
use nalgebra::Vector2;
use rc_ui::atlas::TextureAtlasIndex;
use rc_ui::component::UIComponent;
use rc_ui::elements::button::UIButton;
use rc_ui::elements::image::UIImage;
use rc_ui::elements::rect::UIRect;
use rc_ui::elements::text::UIText;
use rc_ui::elements::UIElement;
use rc_ui::fonts::TextAlignment;
use rc_ui::positioning::{Layout, LayoutScheme};
use specs::{Read, System, WorldExt};

pub struct OptionsScreenComponent {
    layout: Layout,
    pub visible: bool,
}

impl OptionsScreenComponent {
    pub fn new() -> OptionsScreenComponent {
        OptionsScreenComponent {
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

impl UIComponent for OptionsScreenComponent {
    fn render(&self) -> Vec<Box<dyn UIElement + Send + Sync + 'static>> {
        vec![
            UIButton::new(
                Layout {
                    size: Vector2::new(290.0, 60.0),
                    offset: Vector2::new(0.0, 0.0),
                    scheme: LayoutScheme::Left,
                    padding: 0.0,
                },
                String::from("SSAO: Enabled"),
                |universe| {},
            ),
            UIButton::new(
                Layout {
                    size: Vector2::new(290.0, 60.0),
                    offset: Vector2::new(310.0, 00.0),
                    scheme: LayoutScheme::Left,
                    padding: 0.0,
                },
                String::from("Bloom: Enabled"),
                |universe| {},
            ),
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
        self.visible
    }
}
