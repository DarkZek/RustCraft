use bevy::prelude::{App, Event, Plugin};
use nalgebra::Vector3;

pub struct GameEventsPlugin;

impl Plugin for GameEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DestroyBlockEvent>();
    }
}

#[derive(Event, Clone, Copy, Debug)]
pub struct DestroyBlockEvent {
    pub player_triggered: bool,
    pub position: Vector3<i32>,
    pub block_id: u32,
}
