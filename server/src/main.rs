#![feature(fs_try_exists)]
#![feature(let_else)]
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
use crate::events::authorize::AuthorizationEvent;
use crate::game::world::data::WorldData;
use crate::game::world::WorldPlugin;
use crate::systems::chunk::ChunkPlugin;
use crate::systems::tick::tick;
use crate::transport::{TransportPlugin, TransportSystem};
use bevy::app::{App, AppExit, ScheduleRunnerPlugin};
use bevy::log::{info, Level, LogPlugin};
use bevy::prelude::{EventWriter, PluginGroup, PreUpdate, Update};
use bevy::MinimalPlugins;
use rc_networking::types::{ReceivePacket, SendPacket};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

static SHUTDOWN_BIT: AtomicBool = AtomicBool::new(false);

fn main() {
    let _ = ctrlc::set_handler(move || {
        let _ = SHUTDOWN_BIT.store(true, Ordering::Relaxed);
    });

    info!("Rustcraft Server starting up");

    // Build App
    App::default()
        .insert_resource(load_config())
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 200.0,
            ))),
        )
        // Plugins
        .add_plugins(LogPlugin {
            filter: "rechannel=warn".into(),
            level: Level::INFO,
        })
        .add_plugins(WorldPlugin)
        .add_plugins(TransportPlugin)
        .add_plugins(ChunkPlugin)
        // Startup System
        .insert_resource(WorldData::load_spawn_chunks())
        .add_event::<ReceivePacket>()
        .add_event::<SendPacket>()
        .add_event::<AuthorizationEvent>()
        // Receive Server Events
        .add_systems(Update, systems::authorization::authorization_event)
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
