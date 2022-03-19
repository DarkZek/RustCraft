use crate::services::asset_service::AssetService;
use crate::services::settings_service::SettingsService;
use crate::services::ui_service::components::crosshair::CrosshairComponent;
use crate::services::ui_service::components::debug_screen::DebugScreenComponent;
use crate::services::ui_service::components::inventory_bar::InventoryBarComponent;
use crate::services::ui_service::components::options_screen::OptionsScreenComponent;
use crate::services::ui_service::components::pause::PauseMenuComponent;
use std::sync::{Arc, Mutex};

pub mod crosshair;
pub mod debug_screen;
pub mod inventory_bar;
pub mod options_screen;
pub mod pause;

pub struct UIComponents {
    pub crosshair_component: Arc<Mutex<CrosshairComponent>>,
    pub inventory_bar_component: Arc<Mutex<InventoryBarComponent>>,
    pub pause_menu_component: Arc<Mutex<PauseMenuComponent>>,
    pub debug_screen_component: Arc<Mutex<DebugScreenComponent>>,
    pub options_screen_component: Arc<Mutex<OptionsScreenComponent>>,
}

impl UIComponents {
    pub fn new(assets: &AssetService, settings: &SettingsService) -> UIComponents {
        let crosshair_component = Arc::new(Mutex::new(CrosshairComponent::new()));
        let inventory_bar_component = Arc::new(Mutex::new(InventoryBarComponent::new(
            *assets
                .atlas_index
                .as_ref()
                .unwrap()
                .read()
                .unwrap()
                .get("gui/widgets")
                .unwrap(),
        )));
        let pause_menu_component = Arc::new(Mutex::new(PauseMenuComponent::new()));
        let debug_screen_component = Arc::new(Mutex::new(DebugScreenComponent::new()));
        let options_screen_component = Arc::new(Mutex::new(OptionsScreenComponent::new(
            settings,
            pause_menu_component.clone(),
        )));

        UIComponents {
            crosshair_component,
            inventory_bar_component,
            pause_menu_component,
            debug_screen_component,
            options_screen_component,
        }
    }
}

impl Default for UIComponents {
    fn default() -> Self {
        unimplemented!()
    }
}
