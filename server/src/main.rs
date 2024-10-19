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
use crate::game::inventory::propagate_inventories;
use crate::game::update::BlockUpdatePlugin;
use crate::game::world::data::WorldData;
use crate::game::world::WorldPlugin;
use crate::systems::chunk::ChunkPlugin;
use crate::systems::connection::ConnectionPlugin;
use crate::systems::game_object::GameObjectPlugin;
use crate::systems::tick::tick;
use crate::transport::{TransportPlugin, TransportSystem};
use bevy::app::{App, AppExit, ScheduleRunnerPlugin, Startup};
use bevy::log::{info, Level, LogPlugin};
use bevy::prelude::{default, AssetPlugin, AssetServer, EventWriter, PluginGroup, PreUpdate, Res, ResMut, Update, Time, Fixed};
use bevy::MinimalPlugins;
use crate::events::join::PlayerSpawnEvent;
use crate::game::pipes::generate_links;
use rc_networking::types::{ReceivePacket, SendPacket};
use rc_shared::block::{BlockStates, BlockStatesPlugin};
use rc_shared::item::{ItemStates, ItemStatesPlugin};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use rc_networking::protocol::Protocol;
use rc_shared::atlas::{TEXTURE_ATLAS, TextureAtlas};
use rc_shared::PHYSICS_SYNC_RATE_SECONDS;
use crate::game::commands::CommandsPlugin;
use crate::game::entity::EntityPlugin;
use crate::game::join_message::{join_message, leave_message};
use crate::systems::chat::broadcast_chat;

#[macro_use]
extern crate dotenvy_macro;

static SHUTDOWN_BIT: AtomicBool = AtomicBool::new(false);

fn main() {
    let _ = ctrlc::set_handler(move || {
        let _ = SHUTDOWN_BIT.store(true, Ordering::SeqCst);
    });

    TEXTURE_ATLAS.set(TextureAtlas::blank());

    info!("Rustcraft Server starting");

    let assets_dir = dotenv!("ASSETS_DIR");

    // Build App
    App::default()
        .insert_resource(load_config())
        .insert_resource(Time::<Fixed>::from_seconds(PHYSICS_SYNC_RATE_SECONDS))
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 200.0,
            ))),
        )
        .add_plugins(AssetPlugin {
            file_path: assets_dir.to_string(),
            ..default()
        })
        // Plugins
        .add_plugins(LogPlugin {
            filter: "rechannel=warn".into(),
            level: Level::INFO,
            ..default()
        })
        .add_plugins(WorldPlugin)
        .add_plugins(TransportPlugin)
        .add_plugins(ChunkPlugin)
        .add_plugins(GameObjectPlugin)
        .add_plugins(ConnectionPlugin)
        .add_plugins(EntityPlugin)
        .add_plugins(CommandsPlugin)
        .add_plugins(BlockStatesPlugin)
        .add_systems(Startup, load_block_states)
        .add_plugins(ItemStatesPlugin)
        .add_plugins(BlockUpdatePlugin)
        .add_event::<ReceivePacket>()
        .add_event::<SendPacket>()
        .add_event::<AuthorizationEvent>()
        .add_event::<PlayerSpawnEvent>()
        // Gameplay Loop on Tick
        .add_systems(Update, tick)
        .add_systems(PreUpdate, detect_shutdowns)
        .add_systems(Startup, create_states)
        .add_systems(Update, generate_links)
        .add_systems(Update, propagate_inventories)
        .add_systems(Update, (broadcast_chat, join_message, leave_message))
        // Run App
        .run();
}

pub fn detect_shutdowns(
    mut shutdown: EventWriter<AppExit>,
    transport_system: ResMut<TransportSystem>,
    mut send_packet: EventWriter<SendPacket>
) {
    if !SHUTDOWN_BIT.load(Ordering::SeqCst) {
        return;
    }

    // Notify clients
    let packet = Protocol::Disconnect("Server closed.".to_string());
    for client in transport_system.clients.keys() {
        send_packet.send(SendPacket(packet.clone(), *client));
    }

    shutdown.send(AppExit::Success);
    info!("Shutting down connection");
}

pub fn create_states(
    server: Res<AssetServer>,
    mut item_states: ResMut<ItemStates>,
) {
    item_states.load_states("game/state.items".to_string(), &server);
}

pub fn load_block_states(
    mut states: ResMut<BlockStates>
) {
    states.calculate_states()
}
