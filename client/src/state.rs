use bevy::prelude::States;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Preloading,
    Loading,
    MainMenu,
    Connecting,
    InGame,
}
