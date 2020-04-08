use crate::services::asset_service::AssetService;
use crate::services::audio_service::AudioService;
use crate::services::settings_service::SettingsService;
use crate::services::logging_service::LoggingService;
use wgpu::{Device, Queue};

#[macro_use]
pub mod logging_service;
pub mod asset_service;
pub mod audio_service;
pub mod settings_service;

pub struct Services {
    pub(crate) asset: AssetService,
    audio: AudioService,
    settings: SettingsService,
    pub(crate) logging: LoggingService
}

type RenderContext<'a> = (&'a mut Device, &'a mut Queue);

impl Services {
    pub fn load_services(context: RenderContext) -> Services {

        let settings = SettingsService::new();
        let logging = LoggingService::new(&settings);
        let asset = AssetService::new(&settings, context);
        let audio = AudioService::new();

        logging.flush_buffer();

        Services {
            asset,
            audio,
            settings,
            logging
        }
    }
}

