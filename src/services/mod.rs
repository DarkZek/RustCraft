use crate::block::Block;
use crate::render::loading::LoadingScreen;
use crate::services::input_service::InputService;
use crate::services::networking_service::NetworkingService;
use crate::services::{
    asset_service::atlas::atlas_update_blocks,
    asset_service::AssetService,
    audio_service::AudioService,
    chunk_service::ChunkService,
    logging_service::LoggingService,
    settings_service::SettingsService,
    ui_service::{ObjectAlignment, Positioning, UIService},
};
use specs::World;
use std::sync::{Arc, Mutex};
use wgpu::{Device, Queue};
use winit::dpi::PhysicalSize;
use winit::window::Window;

#[macro_use]
pub mod logging_service;
pub mod asset_service;
pub mod audio_service;
pub mod chunk_service;
pub mod input_service;
pub mod networking_service;
pub mod settings_service;
pub mod ui_service;

/// Stores references to important devices needed during initialization of the services.
pub struct ServicesContext<'a> {
    pub device: Arc<Device>,
    pub queue: Arc<Mutex<Queue>>,
    pub blocks: &'a mut Vec<Block>,
    pub size: &'a PhysicalSize<u32>,
    pub window: Arc<Window>,
}

impl<'a> ServicesContext<'_> {
    pub fn new(
        device: Arc<Device>,
        queue: Arc<Mutex<Queue>>,
        blocks: &'a mut Vec<Block>,
        size: &'a PhysicalSize<u32>,
        window: Arc<Window>,
    ) -> ServicesContext<'a> {
        ServicesContext {
            device,
            queue,
            blocks,
            size,
            window,
        }
    }
}

/// Tells all of the services to load in order.
pub fn load_services(mut context: ServicesContext, universe: &mut World) {
    let settings = SettingsService::new();
    let logging = LoggingService::new(&settings);
    LoadingScreen::update_state(10.0);

    let asset = AssetService::new(&settings, &mut context);
    LoadingScreen::update_state(60.0);

    //TODO: Remove this once we have rc_network
    atlas_update_blocks(asset.atlas_index.as_ref().unwrap(), &mut context.blocks);
    let chunk = ChunkService::new(&settings, &mut context, universe);
    LoadingScreen::update_state(80.0);
    let audio = AudioService::new();
    let mut ui = UIService::new(&mut context, &asset, universe);
    let input = InputService::new(&mut context, universe);
    let mut networking_service = NetworkingService::new(universe);
    LoadingScreen::update_state(90.0);

    //TEMP
    //region
    ui.fonts
        .create_text()
        .set_text("Rustcraft V0.01 Alpha")
        .set_size(20.0)
        .set_text_alignment(ObjectAlignment::TopLeft)
        .set_object_alignment(ObjectAlignment::TopLeft)
        .set_positioning(Positioning::Relative)
        .set_background(true)
        .set_offset([0.0, 0.0])
        .build();

    ui.fonts
        .create_text()
        .set_text(&format!(
            "Chunks: {}x16x{} ({} Total)",
            settings.render_distance * 2,
            settings.render_distance * 2,
            (settings.render_distance * 2) * (settings.render_distance * 2) * 16
        ))
        .set_size(20.0)
        .set_text_alignment(ObjectAlignment::TopLeft)
        .set_object_alignment(ObjectAlignment::TopLeft)
        .set_positioning(Positioning::Relative)
        .set_background(true)
        .set_offset([0.0, 60.0])
        .build();
    //endregion

    logging.flush_buffer();

    universe.insert(asset);
    universe.insert(audio);
    universe.insert(settings);
    universe.insert(logging);
    universe.insert(chunk);
    universe.insert(ui);
    universe.insert(input);
    universe.insert(networking_service);
}
