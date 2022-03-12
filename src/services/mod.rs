use crate::render::loading::LoadingScreen;
use crate::services::debugging_service::DebuggingService;
use crate::services::input_service::InputService;
use crate::services::networking_service::NetworkingService;
use crate::services::{
    asset_service::AssetService, audio_service::AudioService, chunk_service::ChunkService,
    settings_service::SettingsService, ui_service::UIService,
};
use rc_logging::LoggingService;
use specs::World;
use std::sync::{Arc, Mutex};
use wgpu::{Queue};
use winit::dpi::PhysicalSize;
use winit::window::Window;

#[macro_use]
pub mod logging_service;
pub mod asset_service;
pub mod audio_service;
pub mod chunk_service;
pub mod debugging_service;
pub mod input_service;
pub mod networking_service;
pub mod settings_service;
pub mod ui_service;

/// Stores references to important devices needed during initialization of the services.
pub struct ServicesContext<'a> {
    pub queue: Arc<Mutex<Queue>>,
    pub size: &'a PhysicalSize<u32>,
    pub window: Arc<Window>,
}

impl<'a> ServicesContext<'_> {
    pub fn new(
        queue: Arc<Mutex<Queue>>,
        size: &'a PhysicalSize<u32>,
        window: Arc<Window>,
    ) -> ServicesContext<'a> {
        ServicesContext {
            queue,
            size,
            window,
        }
    }
}

/// Tells all of the services to load in order.
pub fn load_services(mut context: ServicesContext, universe: &mut World) {
    let settings = SettingsService::new();
    let debugging_service = DebuggingService::new(&settings, universe);

    // Set the logger
    LoggingService::new(&settings.path);

    LoadingScreen::update_state(10.0);

    let asset = AssetService::new(&settings, &mut context);
    LoadingScreen::update_state(60.0);

    let chunk = ChunkService::new(&settings);
    LoadingScreen::update_state(80.0);
    let audio = AudioService::new();
    let ui = UIService::new(&mut context, &asset, universe);
    let input = InputService::new(&mut context, universe);
    let networking_service = NetworkingService::new(universe);
    LoadingScreen::update_state(90.0);

    flush_log!();

    universe.insert(asset);
    universe.insert(audio);
    universe.insert(settings);
    universe.insert(chunk);
    universe.insert(ui);
    universe.insert(input);
    universe.insert(networking_service);
    universe.insert(debugging_service);
}
