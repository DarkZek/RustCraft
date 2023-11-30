use crate::game::chunk::ChunkData;
use crate::game::game_object::GameObject;
use crate::game::transform::Transform;
use crate::game::world::serialized::DeserializedChunkData;
use crate::ServerConfig;
use crate::WorldData;
use bevy::asset::AssetContainer;
use bevy::prelude::{error, info, Commands, Query};
use nalgebra::Vector3;
use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::BufWriter;

impl WorldData {
    pub fn load_spawn_chunks(&mut self, command: &mut Commands) {
        // Load spawn area
        for x in -3..=3 {
            for y in 0..=5 {
                for z in -3..=3 {
                    let pos = Vector3::new(x, y, z);

                    let DeserializedChunkData {
                        version,
                        data,
                        mut game_objects,
                    } = match Self::try_load_chunk(pos) {
                        Ok(Some(chunk)) => chunk,
                        Ok(None) => DeserializedChunkData {
                            version: 0,
                            data: ChunkData::generate(pos),
                            game_objects: vec![],
                        },
                        Err(err) => {
                            error!("Error reading chunk data: {:?}", err);
                            DeserializedChunkData {
                                version: 0,
                                data: ChunkData::generate(pos),
                                game_objects: vec![],
                            }
                        }
                    };

                    assert_eq!(version, 0);

                    self.chunks.insert(Vector3::new(x, y, z), data);

                    for (id, game_object, transform) in game_objects {
                        let entity = command.spawn(transform).insert(game_object).id();

                        self.insert_game_object(id, entity, Vector3::new(x, y, z));
                    }
                }
            }
        }

        info!("Loaded spawn chunks");
    }

    pub fn save_world(&self, config: &ServerConfig, query: &Query<(&GameObject, &Transform)>) {
        create_dir_all("./world/").unwrap();

        for (pos, chunk) in &self.chunks {
            let mut game_objects = vec![];

            for (id, entity) in self.game_objects_chunks.get(pos).unwrap_or(&HashMap::new()) {
                let (game_object, transform) = query.get(*entity).unwrap();
                game_objects.push((*id, game_object.clone(), *transform));
            }

            let data = DeserializedChunkData {
                version: 0,
                data: chunk.clone(),
                game_objects,
            };

            let file = File::create(format!(
                "./world/{:08x}{:08x}{:08x}.chunk",
                pos.x, pos.y, pos.z
            ))
            .unwrap();

            let mut writer = BufWriter::new(file);

            serde_json::to_writer(&mut writer, &data).unwrap();
        }
    }
}
