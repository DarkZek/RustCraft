use bevy::ecs::system::RunSystemOnce;
use bevy::prelude::{EventWriter, info, Res, ResMut, World};
use rc_networking::protocol::clientbound::chat::ChatSent;
use rc_networking::protocol::clientbound::chunk_update::FullChunkUpdate;
use rc_networking::protocol::clientbound::unload_all_chunks::UnloadAllChunks;
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

            format!("Regenerated {chunks} chunks")
        }
        "mod" => {
            let mut config = world.get_resource_mut::<ChunkGenerationConfig>().unwrap();

            let key = command.get(2).unwrap();
            let Ok(value) = command.get(3).unwrap().parse::<f32>() else {
                return format!("Invalid value {:?}", command.get(3).unwrap())
            };

            match key.as_str() {
                "env.terrain_scale" => { config.environment_map_config.terrain_scale = value; }
                "env.vegetation_scale" => { config.environment_map_config.vegetation_scale = value; }
                "env.climate_scale" => { config.environment_map_config.climate_scale = value; }

                "grey.ground_scale_1" => { config.greybox_map_config.ground_scale_1 = value; }
                "grey.ground_scale_2" => { config.greybox_map_config.ground_scale_2 = value; }
                "grey.ground_scale_3" => { config.greybox_map_config.ground_scale_3 = value; }
                "grey.ground_scale_4" => { config.greybox_map_config.ground_scale_4 = value; }
                "grey.cave_scale" => { config.greybox_map_config.cave_scale = value as f64; }
                "grey.hilly_scaler_1" => { config.greybox_map_config.hilly_scaler_1 = value as f64; }
                "grey.hilly_scaler_2" => { config.greybox_map_config.hilly_scaler_2 = value as f64; }
                "grey.terrain_scaler" => { config.greybox_map_config.terrain_scaler = value as f64; }
                "grey.hilly_pow" => { config.greybox_map_config.hilly_pow = value as f64; }
                "grey.ground_scaler_1" => { config.greybox_map_config.ground_scaler_1 = value as f64; }
                "grey.ground_scaler_2" => { config.greybox_map_config.ground_scaler_2 = value as f64; }
                v => {
                    return format!("Invalid key {}", v)
                }
            }

            info!("Set world generator config {}={}", key, value);
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
    resource: Res<ChunkGenerationConfig>,
    mut event_writer: EventWriter<SendPacket>
) {
    info!("Regenerating all chunks");

    // Regenerate all chunks
    for (pos, chunk) in &mut world_data.chunks {
        let data = ChunkData::generate(*pos, &resource);
        *chunk = data;
        chunk.dirty = true;
    }

    for (pos, _) in world_data.chunks_columns.clone() {
        world_data.update_column(pos);
    }

    // Unload all chunks for all users, since we will resend them soon
    for (user, _) in &chunk_system.user_loaded_chunks {
        event_writer.send(SendPacket(
            Protocol::UnloadAllChunks(UnloadAllChunks {
                flag: false
            }),
            *user
        ));
    }
}