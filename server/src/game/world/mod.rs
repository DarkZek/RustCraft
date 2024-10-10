use crate::game::game_object::GameObject;
use crate::game::world::data::WorldData;

use crate::{AppExit, ServerConfig};
use bevy::prelude::*;
use nalgebra::Vector3;
use rc_shared::game_objects::{GameObjectType, ItemDropGameObjectData, PlayerGameObjectData};
use crate::game::inventory::Inventory;
use crate::game::world::column::propagate_chunk_columns;

pub mod data;
mod saving;
pub mod serialized;
pub mod deserialized_player;
pub mod column;

pub static WORLD_SPAWN_LOCATION: Vector3<f32> = Vector3::new(0.0, 20.0, 0.0);

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, save_world)
            .add_systems(Startup, load_spawn_chunks)
            .add_systems(Update, propagate_chunk_columns)
            .insert_resource(WorldData::default());
    }
}

fn save_world(
    world: Res<WorldData>,
    config: Res<ServerConfig>,
    bevy_shutdown: EventReader<AppExit>,
    query: Query<(&GameObject, &GameObjectType, &crate::game::transform::Transform, Option<&Inventory>, Option<&ItemDropGameObjectData>, Option<&PlayerGameObjectData>)>
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

fn load_spawn_chunks(
    mut command: Commands,
    mut world: ResMut<WorldData>,
    config: Res<ServerConfig>
) {
    world.load_spawn_chunks(&mut command, &config);
}
