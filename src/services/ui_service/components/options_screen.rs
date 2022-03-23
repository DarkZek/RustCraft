use crate::render::effects::bloom::BloomPostProcessingEffect;
use crate::render::effects::ssao::SSAOEffect;
use crate::render::effects::EffectPasses;
use crate::render::RenderState;
use crate::services::input_service::actions::ActionSheet;
use crate::services::input_service::input::InputState;
use crate::services::settings_service::SettingsService;
use crate::services::ui_service::components::pause::PauseMenuComponent;
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
use std::sync::{Arc, Mutex};
use winit::monitor::VideoMode;
use winit::window::Fullscreen;

pub struct OptionsScreenComponent {
    layout: Layout,
    pub visible: bool,
    fullscreen: bool,
    ssao: bool,
    bloom: bool,
    edited: bool,
    pause_menu: Arc<Mutex<PauseMenuComponent>>,
}

impl OptionsScreenComponent {
    pub fn new(
        settings: &SettingsService,
        pause_menu: Arc<Mutex<PauseMenuComponent>>,
    ) -> OptionsScreenComponent {
        OptionsScreenComponent {
            layout: Layout::new(
                Vector2::new(600.0, 600.0),
                Vector2::new(0.0, 0.0),
                LayoutScheme::Center,
                0.0,
            ),
            visible: false,
            fullscreen: settings.config.fullscreen,
            ssao: settings.config.ssao,
            bloom: settings.config.bloom,
            edited: false,
            pause_menu,
        }
    }
}

impl UIComponent for OptionsScreenComponent {
    fn name(&self) -> &str {
        "Options Screen"
    }

    fn render(&mut self) -> Vec<Box<dyn UIElement + Send + Sync + 'static>> {
        self.edited = false;

        vec![
            UIButton::new(
                Layout {
                    size: Vector2::new(290.0, 60.0),
                    offset: Vector2::new(0.0, 0.0),
                    scheme: LayoutScheme::TopLeft,
                    padding: 0.0,
                },
                if self.ssao {
                    String::from("SSAO: Enabled")
                } else {
                    String::from("SSAO: Disabled")
                },
                |universe| {
                    let mut ui_component = universe.read_resource::<UIComponents>();
                    let mut options_screen_component =
                        ui_component.options_screen_component.lock().unwrap();

                    options_screen_component.ssao = !options_screen_component.ssao;
                    options_screen_component.edited = true;

                    universe.write_resource::<SettingsService>().config.ssao =
                        options_screen_component.ssao;

                    if options_screen_component.ssao {
                        universe.write_resource::<EffectPasses>().ssao = Some(SSAOEffect::new(
                            &mut universe.write_resource::<RenderState>().queue,
                        ));
                    } else {
                        universe.write_resource::<EffectPasses>().ssao = None;
                    }
                },
            ),
            UIButton::new(
                Layout {
                    size: Vector2::new(290.0, 60.0),
                    offset: Vector2::new(310.0, 00.0),
                    scheme: LayoutScheme::TopLeft,
                    padding: 0.0,
                },
                if self.bloom {
                    String::from("Bloom: Enabled")
                } else {
                    String::from("Bloom: Disabled")
                },
                |universe| {
                    let mut ui_component = universe.read_resource::<UIComponents>();
                    let mut options_screen_component =
                        ui_component.options_screen_component.lock().unwrap();

                    options_screen_component.bloom = !options_screen_component.bloom;
                    options_screen_component.edited = true;

                    universe.write_resource::<SettingsService>().config.bloom =
                        options_screen_component.bloom;

                    if options_screen_component.bloom {
                        universe.write_resource::<EffectPasses>().bloom =
                            Some(BloomPostProcessingEffect::new());
                    } else {
                        universe.write_resource::<EffectPasses>().bloom = None;
                    }
                },
            ),
            UIButton::new(
                Layout {
                    size: Vector2::new(290.0, 60.0),
                    offset: Vector2::new(0.0, 70.0),
                    scheme: LayoutScheme::TopLeft,
                    padding: 0.0,
                },
                if self.fullscreen {
                    String::from("Fullscreen")
                } else {
                    String::from("Windowed")
                },
                |universe| {
                    let mut ui_component = universe.read_resource::<UIComponents>();
                    let mut options_screen_component =
                        ui_component.options_screen_component.lock().unwrap();

                    options_screen_component.edited = true;
                    options_screen_component.fullscreen = !options_screen_component.fullscreen;

                    // Update settings
                    universe
                        .write_resource::<SettingsService>()
                        .config
                        .fullscreen = options_screen_component.fullscreen;

                    if options_screen_component.fullscreen {
                        let mut mode = None;

                        //TODO: Pick better resolution
                        for video_mode in universe
                            .read_resource::<RenderState>()
                            .window
                            .current_monitor()
                            .unwrap()
                            .video_modes()
                        {
                            mode = Some(video_mode);
                            break;
                        }

                        if let Some(mode) = mode {
                            universe
                                .read_resource::<RenderState>()
                                .window
                                .set_fullscreen(Some(Fullscreen::Exclusive(mode)));
                        }
                    } else {
                        universe
                            .read_resource::<RenderState>()
                            .window
                            .set_fullscreen(None);
                    }
                },
            ),
        ]
    }

    fn rerender(&self) -> bool {
        self.edited
    }

    fn positioning(&self) -> &Layout {
        &self.layout
    }

    fn resized(&mut self) {}

    fn back(&mut self) -> bool {
        if self.visible {
            // Toggle
            self.pause_menu.lock().unwrap().visible = true;
            self.visible = false;
            true
        } else {
            false
        }
    }

    fn visible(&self) -> bool {
        self.visible
    }
}
