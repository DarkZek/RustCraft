#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Preloading,
    Loading,
    MainMenu,
    Connecting,
    InGame,
}
