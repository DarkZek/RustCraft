use crate::services::debugging_service::system::DebuggingOverlayElements;
use crate::services::settings_service::SettingsService;
use specs::World;

pub mod system;

pub struct DebuggingService {
    overlay_shown: bool,
}

impl DebuggingService {
    pub fn new(settings: &SettingsService, universe: &mut World) -> DebuggingService {
        universe.insert(DebuggingOverlayElements::default());
        DebuggingService {
            overlay_shown: true,
        }
    }
}
