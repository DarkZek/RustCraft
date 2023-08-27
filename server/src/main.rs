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
use crate::systems::chunk::ChunkPlugin;
use crate::systems::tick::tick;
use crate::transport::{TransportPlugin, TransportSystem};
use bevy::app::{App, AppExit};
use bevy::log::{info, Level, LogPlugin};
use bevy::prelude::{EventWriter, PreUpdate, Update};
use bevy::MinimalPlugins;
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
            level: Level::TRACE,
        })
        .add_plugin(WorldPlugin)
        .add_plugin(TransportPlugin)
        .add_plugin(ChunkPlugin)
        // Startup System
        .insert_resource(WorldData::load_spawn_chunks())
        .add_event::<ReceivePacket>()
        .add_event::<SendPacket>()
        // Receive Server Events
        .add_systems(Update, systems::authorization::authorization_event)
        .add_systems(Update, systems::connection::connection_event)
        .add_systems(Update, systems::disconnect::disconnection_event)
        .add_systems(Update, systems::message::receive_message_event)
        .add_systems(Update, systems::finish_join::detect_finish_join)
        // Gameplay Loop on Tick
        .add_systems(Update, tick)
        .add_systems(PreUpdate, detect_shutdowns)
        // Run App
        .run();
}

pub fn detect_shutdowns(mut shutdown: EventWriter<AppExit>) {
    if SHUTDOWN_BIT.load(Ordering::Relaxed) {
        shutdown.send(AppExit);
        info!("Shutting down server");
    }
}
