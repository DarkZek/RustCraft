use bevy::prelude::{AppExit, EventReader, EventWriter};
use bevy::window::WindowCloseRequested;

pub fn detect_close(
    mut reader: EventReader<WindowCloseRequested>,
    mut exit: EventWriter<AppExit>
) {
    for _ in reader.read() {
        exit.send(AppExit::Success);
    }
}