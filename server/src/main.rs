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
use crate::systems::tick::tick;
use crate::transport::{TransportPlugin, TransportSystem};
use bevy::app::{App, AppExit, CoreStage};
use bevy::ecs::event::EventReader;
use bevy::ecs::prelude::{StageLabel, SystemStage};
use bevy::log::{info, Level, LogPlugin};
use bevy::prelude::Res;
use bevy::MinimalPlugins;
use rc_client::rc_protocol::types::{ReceivePacket, SendPacket};

fn main() {
    info!("Rustcraft Bevy Server Demo starting up");

    // Build App
    App::default()
        .insert_resource(load_config())
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
        // Receive Server Events
        .add_system(systems::authorization::authorization_event)
        .add_system(systems::connection::connection_event)
        .add_system(systems::disconnect::disconnection_event)
        .add_system(systems::message::receive_message_event)
        // Gameplay Loop on Tick
        .add_system(tick)
        .add_system_to_stage(CoreStage::PostUpdate, detect_shutdowns)
        // Run App
        .run();
}

pub fn detect_shutdowns(shutdown: EventReader<AppExit>) {
    if !shutdown.is_empty() {
        println!("Sending disconnect to clients");
    }
}
