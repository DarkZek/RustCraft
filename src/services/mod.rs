use crate::services::asset_service::AssetService;
use crate::services::audio_service::AudioService;
use crate::services::settings_service::SettingsService;
use crate::services::logging_service::LoggingService;
use wgpu::{Device, Queue};
use crate::services::chunk_service::ChunkService;
use crate::block::Block;
use crate::services::asset_service::atlas::atlas_update_blocks;
use crate::services::ui_service::UIService;
use cgmath::Point2;
use winit::dpi::PhysicalSize;

#[macro_use]
pub mod logging_service;
pub mod asset_service;
pub mod audio_service;
pub mod settings_service;
pub mod chunk_service;
pub mod ui_service;

pub struct Services {
    pub asset: AssetService,
    pub audio: AudioService,
    pub settings: SettingsService,
    pub logging: LoggingService,
    pub chunk: ChunkService,
    pub ui: UIService
}

pub struct ServicesContext<'a> {
    pub device: &'a mut Device,
    pub queue: &'a mut Queue,
    pub blocks: &'a mut Vec<Block>,
    pub size: &'a PhysicalSize<u32>
}

impl<'a> ServicesContext<'_> {
    pub fn new(device: &'a mut Device, queue: &'a mut Queue, blocks: &'a mut Vec<Block>, size: &'a PhysicalSize<u32>) -> ServicesContext<'a> {
        ServicesContext {
            device,
            queue,
            blocks,
            size
        }
    }
}

impl Services {
    pub fn load_services(mut context: ServicesContext) -> Services {

        let settings = SettingsService::new();
        let logging = LoggingService::new(&settings);
        let asset = AssetService::new(&settings, &mut context);
        //TODO: Remove this once we have networking
        atlas_update_blocks(asset.atlas_index.as_ref().unwrap(), &mut context.blocks);
        let chunk = ChunkService::new(&settings, &mut context);
        let audio = AudioService::new();
        let mut ui = UIService::new(&mut context, &asset);

        ui.fonts.add_text(String::from("MCv3 for lyfe"), Point2 {
            x: -200.0,
            y: 0.0
        }, 100.0);

        // ui.fonts.add_text(String::from("please help its cold in here"), Point2 {
        //     x: 0.0,
        //     y: -100.0
        // }, 10.0);

        // ui.fonts.add_text(String::from("12345"), Point2 {
        //     x: -0.8,
        //     y: -0.8
        // });

        logging.flush_buffer();

        Services {
            asset,
            audio,
            settings,
            logging,
            chunk,
            ui
        }
    }
}

