use bevy::log::info;
use bevy::prelude::{Event, EventReader, NextState, Res, ResMut, Resource, warn};
use reqwest::Url;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::error::TryRecvError;
use rc_networking::client::NetworkingClient;
use crate::authentication::GameAuthentication;
use crate::state::AppState;
use crate::systems::api::{ApiError, ApiSystem};
use crate::systems::api::join_token::GetJoinTokenResponse;

#[derive(Event)]
pub struct ConnectToServerIntent {
    pub(crate) address: Url
}

#[derive(Resource)]
pub struct PendingServerConnection {
    pending_join_token: Option<UnboundedReceiver<Result<GetJoinTokenResponse, ApiError>>>,
    url: Option<Url>
}

impl PendingServerConnection {
    pub fn new() -> PendingServerConnection {
        PendingServerConnection {
            pending_join_token: None,
            url: None,
        }
    }
}

/// Connects to the local connection instance
pub fn accept_server_connection_intent(
    mut intent: EventReader<ConnectToServerIntent>,
    mut app_state: ResMut<NextState<AppState>>,
    api: ResMut<ApiSystem>,
    mut pending_server_connection: ResMut<PendingServerConnection>,
    game_authentication: Res<GameAuthentication>
) {

    let entry = intent.read().next();

    let Some(intent) = entry else {
        return;
    };

    info!("Connecting to connection on {}", &intent.address);

    app_state.set(AppState::Connecting);

    let response = api.get_join_token(game_authentication.session_token.clone());

    pending_server_connection.pending_join_token = Some(response);
    pending_server_connection.url = Some(intent.address.clone());
}

pub fn connect_to_server(
    mut pending_server_connection: ResMut<PendingServerConnection>,
    mut client: ResMut<NetworkingClient>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    let Some(pending) = &mut pending_server_connection.pending_join_token else {
        return;
    };

    let result = match pending.try_recv() {
        Ok(v) => v,
        Err(TryRecvError::Empty) => {
            return;
        }
        Err(TryRecvError::Disconnected) => {
            pending_server_connection.pending_join_token.take();
            // TODO: Handle when the api connection fails
            unimplemented!("Api connection failed");
        }
    };

    pending_server_connection.pending_join_token.take();

    let response = match result {
        Ok(v) => v,
        Err(e) => {
            warn!("Failed fetch join token: {:?}", e);
            app_state.set(AppState::MainMenu);
            return
        }
    };

    let url = pending_server_connection.url.take().unwrap();

    client.connect(url, response.join_token);
}