use bevy::app::{App, Update};
use bevy::prelude::{in_state, IntoSystemConfigs, Plugin};
use crate::state::AppState;
use crate::systems::connection::connect::{accept_server_connection_intent, connect_to_server, ConnectToServerIntent, PendingServerConnection};
use crate::systems::connection::connection_complete::{detect_connection_complete};

pub(crate) mod connect;
mod connection_complete;

pub struct ConnectionPlugin;

impl Plugin for ConnectionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ConnectToServerIntent>()
            .insert_resource(PendingServerConnection::new())
            // Once the game is in the Main Menu connect to connection as we have no main screen yet
            .add_systems(Update, (accept_server_connection_intent, connect_to_server))
            .add_systems(Update, detect_connection_complete.run_if(in_state(AppState::Connecting)));
    }
}