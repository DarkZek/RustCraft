use crate::state::AppState;
use bevy::prelude::*;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::UnboundedReceiver;
use crate::systems::api::ApiSystem;
use crate::authentication::GameAuthentication;
use crate::systems::api::ApiError;
use crate::systems::api::open_session::OpenSessionResponse;

#[derive(Default)]
pub struct LoadingLocal {
    session: Option<UnboundedReceiver<Result<OpenSessionResponse, ApiError>>>
}

pub fn set_loading(
    mut app_state: ResMut<NextState<AppState>>,
    mut local: Local<LoadingLocal>,
    api: ResMut<ApiSystem>,
    mut game_authentication: ResMut<GameAuthentication>
) {
    if local.session.is_none() {
        // Fetch session
        let token = api.open_session(game_authentication.refresh_token.clone());
        local.session = Some(token);
        return;
    }

    let Some(session) = &mut local.session else {
        return;
    };

    let data = match session.try_recv() {
        Ok(v) => v,
        Err(TryRecvError::Empty) => return,
        Err(TryRecvError::Disconnected) => {
            warn!("Failed opening session. No response");
            std::process::exit(1);
        }
    };

    let data = match data {
        Ok(v) => v,
        Err(e) => {
            warn!("Api failed opening session. {:?}", e);
            std::process::exit(1);
        }
    };

    info!("Successfully fetched session token");

    game_authentication.session_token = data.session_token;

    app_state.set(AppState::Loading);
}

#[derive(Resource, Default, Debug)]
pub struct LoadingUIData {
    pub texture_atlas: bool,
    pub block_states: bool,
    pub item_states: bool,
    pub ui: Option<Entity>,
}

pub fn setup_loading_ui(mut commands: Commands, mut data: ResMut<LoadingUIData>) {
    let ui = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexEnd,
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: Color::srgb(0.361, 0.42, 0.753).into(),
            ..default()
        })
        .id();

    data.ui = Some(ui);
}

pub fn remove_loading_ui(mut commands: Commands, data: ResMut<LoadingUIData>) {
    if let Some(ui) = data.ui {
        commands.entity(ui).despawn();
    }
    commands.remove_resource::<LoadingUIData>();
}

pub fn check_loading(
    data: Res<LoadingUIData>,
    app_state: ResMut<State<AppState>>,
    mut set_app_state: ResMut<NextState<AppState>>,
) {
    if !data.is_changed() {
        return;
    }

    // Once every part is done deserialisation, show the main menu
    if data.texture_atlas && data.block_states && data.item_states {
        // If we're still in deserialisation mode, the block states being loaded means we're ready for the main menu. This may be changed in the future
        if *app_state == AppState::Loading {
            set_app_state.set(AppState::MainMenu);
        }
    }
}
