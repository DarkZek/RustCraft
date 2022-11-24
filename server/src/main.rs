#![feature(fs_try_exists)]
#![allow(unused_variables)]
#![allow(dead_code)]

pub mod config;
pub mod console;
pub mod error;
pub mod events;
pub mod game;
pub mod helpers;
mod systems;
pub mod transport;

use crate::config::{load_config, ServerConfig};
use crate::game::world::data::WorldData;
use crate::game::world::WorldPlugin;
use crate::systems::tick::tick;
use crate::transport::{TransportPlugin, TransportSystem};
use bevy::app::{App, AppExit, CoreStage};
use bevy::log::{info, Level, LogPlugin};
use bevy::prelude::{EventWriter};
use bevy::{MinimalPlugins};
use rc_networking::types::{ReceivePacket, SendPacket};
use std::sync::atomic::{AtomicBool, Ordering};

static SHUTDOWN_BIT: AtomicBool = AtomicBool::new(false);

fn main() {
    let _ = ctrlc::set_handler(move || {
        let _ = SHUTDOWN_BIT.store(true, Ordering::Relaxed);
    });

    info!("Rustcraft Server starting up");

    // Build App
    App::default()
        .insert_resource(load_config())
        .add_plugins(MinimalPlugins)
        // Plugins
        .add_plugin(LogPlugin {
            filter: "rechannel=warn".into(),
            level: Level::DEBUG,
        })
        .add_plugin(WorldPlugin)
        .add_plugin(TransportPlugin)
        // Startup System
        .insert_resource(WorldData::load_spawn_chunks())
        .add_event::<ReceivePacket>()
        .add_event::<SendPacket>()
        // Receive Server Events
        .add_system(systems::authorization::authorization_event)
        .add_system(systems::connection::connection_event)
        .add_system(systems::disconnect::disconnection_event)
        .add_system(systems::message::receive_message_event)
        // Gameplay Loop on Tick
        .add_system(tick)
        .add_system_to_stage(CoreStage::PreUpdate, detect_shutdowns)
        // Run App
        .run();
}

pub fn detect_shutdowns(mut shutdown: EventWriter<AppExit>) {
    if SHUTDOWN_BIT.load(Ordering::Relaxed) {
        shutdown.send(AppExit);
        info!("Shutting down server");
    }
}
