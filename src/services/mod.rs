use crate::services::asset_service::AssetService;
use crate::services::audio_service::AudioService;
use crate::services::settings_service::SettingsService;
use crate::services::logging_service::LoggingService;
use wgpu::{Device, Queue};
use crate::services::chunk_service::ChunkService;
use crate::block::Block;
use crate::services::asset_service::atlas::atlas_update_blocks;

#[macro_use]
pub mod logging_service;
pub mod asset_service;
pub mod audio_service;
pub mod settings_service;
pub mod chunk_service;

pub struct Services {
    pub asset: AssetService,
    pub audio: AudioService,
    pub settings: SettingsService,
    pub logging: LoggingService,
    pub chunk: ChunkService
}

pub struct ServicesContext<'a> {
    pub device: &'a mut Device,
    pub queue: &'a mut Queue,
    pub blocks: &'a mut Vec<Block>
}

impl<'a> ServicesContext<'_> {
    pub fn new(device: &'a mut Device, queue: &'a mut Queue, blocks: &'a mut Vec<Block>) -> ServicesContext<'a> {
        ServicesContext {
            device,
            queue,
            blocks
        }
    }
}

impl Services {
    pub fn load_services(mut context: ServicesContext) -> Services {

        let settings = SettingsService::new();
        let logging = LoggingService::new(&settings);
        let asset = AssetService::new(&settings, &mut context);
        //TODO: Remove this once we have networking
        atlas_update_blocks(asset.texture_atlas_index.as_ref().unwrap(), &mut context.blocks);
        let chunk = ChunkService::new(&settings, &mut context);
        let audio = AudioService::new();

        logging.flush_buffer();

        Services {
            asset,
            audio,
            settings,
            logging,
            chunk
        }
    }
}

