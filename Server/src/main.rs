mod systems;
mod resources;
pub mod game;
pub mod helpers;

use bevy_app::{App, ScheduleRunnerPlugin};
use bevy_core::CorePlugin;
use bevy_log::{info, LogPlugin};
use bevy_testing_protocol::channels::Channels;
use bevy_testing_protocol::config::network_config;
use bevy_testing_protocol::protocol::{Protocol, ProtocolKind};
use naia_bevy_server::{Plugin, ServerConfig, Stage};
use crate::systems::init::init;
use crate::systems::tick::tick;

fn main() {
    info!("Naia Bevy Server Demo starting up");

    // Build App
    App::default()
        // Plugins
        .add_plugin(CorePlugin::default())
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_plugin(LogPlugin::default())
        .add_plugin(Plugin::<Protocol, Channels>::new(
            ServerConfig::default(),
            network_config(),
        ))
        // Startup System
        .add_startup_system(init)
        // Receive Server Events
        .add_system_to_stage(Stage::ReceiveEvents, systems::authorization::authorization_event)
        .add_system_to_stage(Stage::ReceiveEvents, systems::connection::connection_event)
        .add_system_to_stage(Stage::ReceiveEvents, systems::disconnect::disconnection_event)
        .add_system_to_stage(Stage::ReceiveEvents, systems::message::receive_message_event)
        // Gameplay Loop on Tick
        .add_system_to_stage(Stage::Tick, tick)
        // Run App
        .run();
}
