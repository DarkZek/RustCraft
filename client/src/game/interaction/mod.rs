use bevy::app::{App, Update};
use bevy::prelude::Plugin;
use crate::game::interaction::destroy::mouse_interaction_destroy;
use crate::game::interaction::place::mouse_interaction_place;

pub mod highlight;
mod destroy;
mod place;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (mouse_interaction_destroy, mouse_interaction_place));
    }
}