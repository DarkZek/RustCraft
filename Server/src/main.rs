pub mod error;
pub mod events;
pub mod game;
pub mod helpers;
mod resources;
mod systems;
pub mod transport;

use crate::error::ServerError;
use crate::resources::World;
use crate::systems::tick::tick;
use crate::transport::packet::{ReceivePacket, SendPacket};
use crate::transport::{TransportPlugin, TransportSystem};
use bevy_app::{App, ScheduleRunnerPlugin};
use bevy_core::CorePlugin;
use bevy_ecs::prelude::{StageLabel, State, SystemStage};
use bevy_log::{debug, info, Level, LogPlugin, LogSettings};
use std::net::IpAddr;
use std::process::exit;
use std::str::FromStr;

fn main() {
    info!("Rustcraft Bevy Server Demo starting up");

    // Build App
    App::default()
        .insert_resource(LogSettings {
            filter: "".into(),
            level: Level::TRACE,
        })
        // Plugins
        .add_plugin(CorePlugin::default())
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_plugin(LogPlugin::default())
        .add_plugin(TransportPlugin)
        // Startup System
        .insert_resource(World::new())
        .add_event::<ReceivePacket>()
        .add_event::<SendPacket>()
        .add_stage(ServerState::Networking, SystemStage::single_threaded())
        // Receive Server Events
        .add_system_to_stage(
            ServerState::Networking,
            systems::authorization::authorization_event,
        )
        .add_system_to_stage(
            ServerState::Networking,
            systems::connection::connection_event,
        )
        .add_system_to_stage(
            ServerState::Networking,
            systems::disconnect::disconnection_event,
        )
        .add_system_to_stage(
            ServerState::Networking,
            systems::message::receive_message_event,
        )
        // Gameplay Loop on Tick
        .add_system_to_stage(ServerState::Networking, tick)
        // Run App
        .run();
}

#[derive(StageLabel)]
enum ServerState {
    Networking,
}
