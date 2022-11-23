#![allow(unused_variables)]
#![allow(dead_code)]

pub mod console;
pub mod error;
pub mod events;
pub mod game;
pub mod helpers;
mod resources;
mod systems;
pub mod transport;


use crate::resources::WorldData;
use crate::systems::tick::tick;
use crate::transport::{TransportPlugin, TransportSystem};
use bevy::app::{App, AppExit, CoreStage};
use bevy::ecs::event::EventReader;
use bevy::ecs::prelude::{StageLabel, SystemStage};
use bevy::log::{info, Level, LogPlugin};
use bevy::MinimalPlugins;
use rc_networking::types::{ReceivePacket, SendPacket};

fn main() {
    info!("Rustcraft Bevy Server Demo starting up");

    // Build App
    App::default()
        .add_plugins(MinimalPlugins)
        // Plugins
        .add_plugin(LogPlugin {
            filter: "".into(),
            level: Level::DEBUG,
        })
        .add_plugin(TransportPlugin)
        // Startup System
        .insert_resource(WorldData::new())
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
        .add_system_to_stage(CoreStage::PostUpdate, detect_shutdowns)
        // Run App
        .run();
}

#[derive(StageLabel)]
enum ServerState {
    Networking,
}

pub fn detect_shutdowns(shutdown: EventReader<AppExit>) {
    if !shutdown.is_empty() {
        println!("Sending disconnect to server");
    }
}
