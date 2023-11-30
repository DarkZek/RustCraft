use crate::game::game_object::GameObject;
use crate::game::world::data::WorldData;
use crate::game::world::serialized::DeserializedChunkData;
use crate::{App, AppExit, ServerConfig};
use bevy::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufWriter;

pub mod data;
mod saving;
pub mod serialized;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, save_world)
            .add_systems(Startup, load_spawn_chunks)
            .insert_resource(WorldData::default());
    }
}

fn save_world(
    world: Res<WorldData>,
    config: Res<ServerConfig>,
    bevy_shutdown: EventReader<AppExit>,
    query: Query<(&GameObject, &crate::game::transform::Transform)>,
) {
    if bevy_shutdown.is_empty() {
        return;
    }

    if !config.save_world {
        info!("Not saving world due to config");
        return;
    }

    info!("Saving world...");

    world.save_world(&config, &query);

    info!("Saved world.");
}

fn load_spawn_chunks(mut command: Commands, mut world: ResMut<WorldData>) {
    world.load_spawn_chunks(&mut command);
}
