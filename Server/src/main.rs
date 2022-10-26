mod systems;
mod resources;
pub mod game;
pub mod helpers;
pub mod transport;
pub mod error;
pub mod events;

use std::net::IpAddr;
use std::process::exit;
use std::str::FromStr;
use bevy_app::{App, ScheduleRunnerPlugin};
use bevy_core::CorePlugin;
use bevy_log::{info, LogPlugin};
use crate::error::ServerError;
use crate::systems::tick::tick;
use crate::transport::{TransportPlugin, TransportSystem};
use bevy_ecs::prelude::{StageLabel, State, SystemStage};
use rustcraft_protocol::protocol::{ReceivePacket, SendPacket};
use crate::resources::World;

fn main() {
    info!("Rustcraft Bevy Server Demo starting up");

    // Build App
    App::default()
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
        .add_system_to_stage(ServerState::Networking, systems::authorization::authorization_event)
        .add_system_to_stage(ServerState::Networking, systems::connection::connection_event)
        .add_system_to_stage(ServerState::Networking, systems::disconnect::disconnection_event)
        //.add_system_to_stage(ServerState::Networking, systems::message::receive_message_event)
        // Gameplay Loop on Tick
        .add_system_to_stage(ServerState::Networking, tick)
        // Run App
        .run();
}

#[derive(StageLabel)]
enum ServerState {
    Networking,
}