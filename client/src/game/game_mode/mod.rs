use bevy::prelude::*;
use rc_shared::game_mode::PlayerGameMode;

pub struct GameModePlugin;

impl Plugin for GameModePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerGameMode::Play);
    }
}