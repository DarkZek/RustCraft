use nalgebra::Vector2;
use specs::WorldExt;

use rc_ui::component::UIComponent;
use rc_ui::elements::button::UIButton;

use rc_ui::elements::UIElement;

use crate::services::input_service::input::InputState;
use crate::services::networking_service::NetworkingService;
use crate::services::settings_service::SettingsService;
use crate::services::ui_service::components::UIComponents;
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
    fn render(&mut self) -> Vec<Box<dyn UIElement + Send + Sync + 'static>> {
        vec![
            UIButton::new(
                Layout {
                    size: Vector2::new(600.0, 60.0),
                    offset: Vector2::new(0.0, 0.0),
                    scheme: LayoutScheme::Top,
                    padding: 0.0,
                },
                String::from("Back To Game"),
                |universe| {
                    universe
                        .read_resource::<UIComponents>()
                        .pause_menu_component
                        .lock()
                        .unwrap()
                        .visible = false;
                    universe.write_resource::<InputState>().set_capture_mouse();
                },
            ),
            UIButton::new(
                Layout {
                    size: Vector2::new(600.0, 60.0),
                    offset: Vector2::new(0.0, 80.0),
                    scheme: LayoutScheme::Top,
                    padding: 0.0,
                },
                String::from("Options"),
                |universe| {
                    universe
                        .read_resource::<UIComponents>()
                        .pause_menu_component
                        .lock()
                        .unwrap()
                        .visible = false;
                    universe
                        .read_resource::<UIComponents>()
                        .options_screen_component
                        .lock()
                        .unwrap()
                        .visible = true;
                },
            ),
            UIButton::new(
                Layout {
                    size: Vector2::new(600.0, 60.0),
                    offset: Vector2::new(0.0, 160.0),
                    scheme: LayoutScheme::Top,
                    padding: 0.0,
                },
                String::from("Exit Game"),
                |universe| {
                    universe.read_resource::<NetworkingService>().shutdown();
                    universe.read_resource::<SettingsService>().save();
                    std::process::exit(0);
                },
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

    fn back(&mut self) -> bool {
        false
    }

    fn visible(&self) -> bool {
        self.visible
    }
}
