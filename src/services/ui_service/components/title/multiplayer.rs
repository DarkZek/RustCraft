use nalgebra::Vector2;
use rc_ui::ATLAS_INDEXES;
use specs::WorldExt;

use rc_ui::component::UIComponent;
use rc_ui::elements::button::UIButton;
use rc_ui::elements::image::UIImage;
use rc_ui::elements::text::UIText;

use rc_ui::elements::UIElement;
use rc_ui::fonts::TextAlignment;

use crate::services::input_service::input::InputState;
use crate::services::networking_service::NetworkingService;
use crate::services::settings_service::SettingsService;
use crate::services::ui_service::components::title::main::MainTitleScreenComponent;
use crate::services::ui_service::components::UIComponents;
use rc_ui::positioning::{Layout, LayoutScheme};

pub struct MultiplayerConnectionScreenComponent {
    layout: Layout,
    pub visible: bool,
    ip: String,
}

impl MultiplayerConnectionScreenComponent {
    pub fn new() -> MultiplayerConnectionScreenComponent {
        MultiplayerConnectionScreenComponent {
            layout: Layout::new(
                Vector2::new(650.0, 600.0),
                Vector2::new(0.0, 0.0),
                LayoutScheme::Center,
                0.0,
            ),
            visible: false,
            ip: String::from("localhost:25565"),
        }
    }
}

impl UIComponent for MultiplayerConnectionScreenComponent {
    fn name(&self) -> &str {
        "Multiplayer Connection Screen"
    }

    fn render(&mut self) -> Vec<Box<dyn UIElement + Send + Sync + 'static>> {
        vec![
            UIButton::new(
                Layout {
                    size: Vector2::new(600.0, 60.0),
                    offset: Vector2::new(25.0, 160.0),
                    scheme: LayoutScheme::TopLeft,
                    padding: 0.0,
                },
                String::from("Singleplayer"),
                |universe| {},
            ),
            UIButton::new(
                Layout {
                    size: Vector2::new(600.0, 60.0),
                    offset: Vector2::new(25.0, 240.0),
                    scheme: LayoutScheme::TopLeft,
                    padding: 0.0,
                },
                String::from("Connect"),
                |universe| {
                    universe
                        .read_resource::<UIComponents>()
                        .multiplayer_connection_screen_component
                        .lock()
                        .unwrap()
                        .visible = false;
                    universe
                        .read_resource::<UIComponents>()
                        .crosshair_component
                        .lock()
                        .unwrap()
                        .visible = true;
                    universe
                        .read_resource::<UIComponents>()
                        .inventory_bar_component
                        .lock()
                        .unwrap()
                        .visible = true;

                    let mut port = 25565;
                    let mut ip = universe
                        .read_resource::<UIComponents>()
                        .multiplayer_connection_screen_component
                        .lock()
                        .unwrap()
                        .ip
                        .clone();

                    if ip.contains(":") {
                        let mut split = ip.split(":").collect::<Vec<&str>>();
                        if let Some(str) = split.get(1) {
                            if let Ok(val) = str.parse::<u32>() {
                                port = val;
                                ip = String::from(*split.get(0).unwrap());
                            }
                        }
                    }

                    universe
                        .write_resource::<NetworkingService>()
                        .connect_to_server(ip, port)
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
