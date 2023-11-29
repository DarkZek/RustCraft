#![feature(fs_try_exists)]
#![feature(let_else)]
#![allow(unused_variables)]
#![allow(dead_code)]

pub mod config;
pub mod dummy_atlas;
pub mod error;
pub mod events;
pub mod game;
pub mod helpers;
pub mod systems;
pub mod transport;

use crate::config::{load_config, ServerConfig};
use crate::dummy_atlas::DummyAtlas;
use crate::events::authorize::AuthorizationEvent;
use crate::game::update::BlockUpdatePlugin;
use crate::game::world::data::WorldData;
use crate::game::world::WorldPlugin;
use crate::systems::chunk::ChunkPlugin;
use crate::systems::connection::ConnectionPlugin;
use crate::systems::game_object::GameObjectPlugin;
use crate::systems::tick::tick;
use crate::transport::{TransportPlugin, TransportSystem};
use bevy::app::{App, AppExit, ScheduleRunnerPlugin};
use bevy::log::{info, Level, LogPlugin};
use bevy::prelude::{default, AssetPlugin, EventWriter, PluginGroup, PreUpdate, Update};
use bevy::MinimalPlugins;
use rc_networking::client::systems::detect_shutdown_system;
use rc_networking::types::{ReceivePacket, SendPacket};
use rc_shared::block::BlockStatesPlugin;
use rc_shared::item::ItemStatesPlugin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

static SHUTDOWN_BIT: AtomicBool = AtomicBool::new(false);
static DUMMY_ATLAS: DummyAtlas = DummyAtlas;

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
        .add_plugins(AssetPlugin {
            file_path: "../assets".to_string(),
            ..default()
        })
        // Plugins
        .add_plugins(LogPlugin {
            filter: "rechannel=warn".into(),
            level: Level::INFO,
        })
        .add_plugins(WorldPlugin)
        .add_plugins(TransportPlugin)
        .add_plugins(ChunkPlugin)
        .add_plugins(GameObjectPlugin)
        .add_plugins(ConnectionPlugin)
        .add_plugins(BlockStatesPlugin {
            texture_atlas: &DUMMY_ATLAS,
        })
        .add_plugins(ItemStatesPlugin)
        .add_plugins(BlockUpdatePlugin)
        // Startup System
        .insert_resource(WorldData::load_spawn_chunks())
        .add_event::<ReceivePacket>()
        .add_event::<SendPacket>()
        .add_event::<AuthorizationEvent>()
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
