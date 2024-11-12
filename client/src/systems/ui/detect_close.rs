use bevy::prelude::{AppExit, EventReader, EventWriter, NextState, ResMut};
use bevy::window::WindowCloseRequested;
use crate::state::AppState;

pub fn detect_close(
    mut reader: EventReader<WindowCloseRequested>,
    mut exit: EventWriter<AppExit>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for _ in reader.read() {
        // exit.send(AppExit::Success);
        app_state.set(AppState::Shutdown);
    }
}