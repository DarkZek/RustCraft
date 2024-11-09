use bevy::ecs::system::RunSystemOnce;
use bevy::prelude::{EventWriter, info, Res, ResMut, World};
use rc_networking::protocol::clientbound::chunk_update::FullChunkUpdate;
use rc_networking::protocol::Protocol;
use rc_networking::types::SendPacket;
use rc_shared::constants::UserId;
use crate::game::chunk::ChunkData;
use crate::game::generation::ChunkGenerationConfig;
use crate::game::world::data::WorldData;
use crate::systems::chunk::ChunkSystem;

pub fn parse_world_gen(
    command: Vec<String>,
    user_id: UserId,
    world: &mut World
) -> String {
    if command.len() == 1 {
        return format!("Incorrect arguments for command <{}>. Options: regen, mod", command.get(0).unwrap());
    }

    match command.get(1).unwrap().as_str() {
        "regen" => {
            let chunks = world.get_resource::<WorldData>().unwrap().chunks.len();

            world.run_system_once(reload_chunks);

            format!("Reloading {chunks} chunks")
        }
        "mod" => {
            let mut config = world.get_resource_mut::<ChunkGenerationConfig>().unwrap();

            let key = command.get(2).unwrap();
            let Ok(value) = command.get(3).unwrap().parse::<f32>() else {
                return format!("Invalid value {:?}", command.get(3).unwrap())
            };

            match key.as_str() {
                "env.terrain_scale" => {
                    config.environment_map_config.terrain_scale = value;
                }
                "env.vegetation_scale" => {
                    config.environment_map_config.vegetation_scale = value;
                }
                "env.climate_scale" => {
                    config.environment_map_config.climate_scale = value;
                }
                v => {
                    return format!("Invalid key {}", v)
                }
            }

            format!("Modified config {} to {}", key, value)
        }
        v => {
            format!("Unknown argument {}", v)
        }
    }
}

fn reload_chunks(
    mut world_data: ResMut<WorldData>,
    chunk_system: Res<ChunkSystem>,
    resource: Res<ChunkGenerationConfig>
) {
    info!("Regenerating all chunks");

    for (pos, chunk) in &mut world_data.chunks {
        let data = ChunkData::generate(*pos, &resource);
        *chunk = data;
        chunk.dirty = true;
    }
}