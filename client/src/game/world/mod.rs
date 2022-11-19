use crate::game::world::sun::setup_sun;
use bevy::app::App;
use bevy::prelude::Plugin;

pub mod sun;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_sun);
    }
}
