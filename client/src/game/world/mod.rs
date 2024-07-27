use crate::game::world::destroy_block::destroy_block_system;
use crate::game::world::sun::{setup_sun, update_sun};
use bevy::app::{App, Startup};
use bevy::prelude::{AssetApp, IntoSystemConfigs, Plugin, Update};

mod destroy_block;
pub mod sun;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_sun)
            .add_systems(
                Update,
                (update_sun, destroy_block_system),
            );
    }
}
