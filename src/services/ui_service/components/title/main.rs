use nalgebra::Vector2;
use rc_ui::ATLAS_INDEXES;
use specs::WorldExt;

use rc_ui::component::UIComponent;
use rc_ui::elements::button::UIButton;
use rc_ui::elements::image::UIImage;
use rc_ui::elements::text::UIText;

use rc_ui::elements::UIElement;
use rc_ui::fonts::TextAlignment;

use crate::game::game_state::{GameState, ProgramState};
use crate::services::input_service::input::InputState;
use crate::services::networking_service::NetworkingService;
use crate::services::settings_service::SettingsService;
use crate::services::ui_service::components::UIComponents;
use rc_ui::positioning::{Layout, LayoutScheme};

pub struct MainTitleScreenComponent {
    layout: Layout,
    pub visible: bool,
}

impl MainTitleScreenComponent {
    pub fn new() -> MainTitleScreenComponent {
        MainTitleScreenComponent {
            layout: Layout::new(
                Vector2::new(650.0, 600.0),
                Vector2::new(0.0, 0.0),
                LayoutScheme::Center,
                0.0,
            ),
            visible: true,
        }
    }
}

impl UIComponent for MainTitleScreenComponent {
    fn name(&self) -> &str {
        "Main Title Screen"
    }

    fn render(&mut self) -> Vec<Box<dyn UIElement + Send + Sync + 'static>> {
        vec![
            UIImage::new(
                Layout {
                    size: Vector2::new(553.0, 100.0),
                    offset: Vector2::new(0.0, 0.0),
                    scheme: LayoutScheme::Top,
                    padding: 0.0,
                },
                *ATLAS_INDEXES
                    .get()
                    .unwrap()
                    .read()
                    .unwrap()
                    .get("gui/title/rustcraft")
                    .unwrap(),
            ),
            Box::new(
                UIButton::new(
                    Layout {
                        size: Vector2::new(600.0, 60.0),
                        offset: Vector2::new(25.0, 160.0),
                        scheme: LayoutScheme::TopLeft,
                        padding: 0.0,
                    },
                    String::from("Singleplayer"),
                    |universe| {},
                )
                .with_disabled(true),
            ),
            UIButton::new(
                Layout {
                    size: Vector2::new(600.0, 60.0),
                    offset: Vector2::new(25.0, 240.0),
                    scheme: LayoutScheme::TopLeft,
                    padding: 0.0,
                },
                String::from("Multiplayer"),
                |universe| {
                    universe
                        .read_resource::<UIComponents>()
                        .main_title_screen_component
                        .lock()
                        .unwrap()
                        .visible = false;
                    universe
                        .read_resource::<UIComponents>()
                        .multiplayer_connection_screen_component
                        .lock()
                        .unwrap()
                        .visible = true;
                },
            ),
            UIButton::new(
                Layout {
                    size: Vector2::new(290.0, 60.0),
                    offset: Vector2::new(25.0, 320.0),
                    scheme: LayoutScheme::TopLeft,
                    padding: 0.0,
                },
                String::from("Options"),
                |universe| {
                    universe
                        .read_resource::<UIComponents>()
                        .main_title_screen_component
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
                    size: Vector2::new(290.0, 60.0),
                    offset: Vector2::new(335.0, 320.0),
                    scheme: LayoutScheme::TopLeft,
                    padding: 0.0,
                },
                String::from("Exit Game"),
                |universe| {
                    universe.read_resource::<NetworkingService>().shutdown();
                    universe.read_resource::<SettingsService>().save();
                    std::process::exit(0);
                },
            ),
            Box::new(UIText {
                text: format!("Rustcraft v{}", env!("CARGO_PKG_VERSION")),
                font_size: 20.0,
                color: [1.0; 4],
                layout: Layout {
                    size: Vector2::new(200.0, 20.0),
                    offset: Vector2::zeros(),
                    scheme: LayoutScheme::BottomLeft,
                    padding: 5.0,
                },
                alignment: TextAlignment::Left,
                background_color: [0.0; 4],
            }),
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
