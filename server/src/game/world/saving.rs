use crate::game::chunk::ChunkData;
use crate::game::game_object::GameObject;
use crate::game::transform::Transform;
use crate::game::world::data::GAME_OBJECT_ID_COUNTER;
use crate::game::world::serialized::DeserializedChunkData;
use crate::ServerConfig;
use crate::WorldData;
use bevy::log::{error, info};
use bevy::prelude::{Commands, Query};
use nalgebra::Vector3;
use std::collections::HashMap;
use std::fs;
use std::fs::{create_dir_all, File};
use std::io::BufWriter;
use std::str::FromStr;
use std::sync::atomic::Ordering;
use rc_shared::game_objects::{DebugGameObjectData, GameObjectData, GameObjectType, ItemDropGameObjectData, PlayerGameObjectData};
use crate::config::WorldType;
use crate::game::inventory::Inventory;
use crate::game::world::deserialized_player::DeserializedPlayerData;

impl WorldData {
    pub fn load_spawn_chunks(
        &mut self,
        command: &mut Commands,
        config: &ServerConfig
    ) {
        // Load spawn area
        for x in -3..=3 {
            for y in 0..=5 {
                for z in -3..=3 {
                    let pos = Vector3::new(x, y, z);

                    let DeserializedChunkData {
                        version,
                        data,
                        game_objects,
                    } = match Self::try_load_chunk(pos) {
                        Ok(Some(chunk)) => Some(chunk),
                        Ok(None) => None,
                        Err(err) => {
                            error!("Error reading chunk data: {:?}", err);
                            None
                        }
                    }.unwrap_or_else(|| {
                        // Generate the chunk
                        let data = match config.world_type {
                            WorldType::Regular => ChunkData::generate(pos),
                            WorldType::Canvas => ChunkData::generate_canvas(pos)
                        };
                        DeserializedChunkData {
                            version: 0,
                            data,
                            game_objects: vec![],
                        }
                    });

                    assert_eq!(version, 0);

                    self.insert_chunk(data);

                    for (id, game_object, transform, data) in game_objects {
                        let mut entity_commands = command.spawn(transform);

                        entity_commands.insert(game_object);

                        match data {
                            GameObjectData::Debug => entity_commands.insert(DebugGameObjectData),
                            GameObjectData::ItemDrop(data) => entity_commands.insert(data),
                            GameObjectData::Player(data) => entity_commands.insert(data)
                        };

                        let entity = entity_commands.id();
                        self.insert_game_object(id, entity, Vector3::new(x, y, z));
                    }
                }
            }
        }

        // Load sequential object id counter
        if fs::exists("./world/game_objects").unwrap() {
            let entites = u64::from_str(&fs::read_to_string("./world/game_objects").unwrap())
                .expect("'/world/game_objects' file corrupted");

            GAME_OBJECT_ID_COUNTER.store(entites, Ordering::SeqCst);
        }

        info!("Loaded spawn chunks");
    }

    pub fn save_world(
        &self,
        config: &ServerConfig,
        query: &Query<(&GameObject, &GameObjectType, &Transform, Option<&Inventory>, Option<&ItemDropGameObjectData>, Option<&PlayerGameObjectData>)>
    ) {
        create_dir_all("./world/chunks").unwrap();
        create_dir_all("./world/players").unwrap();

        // Write chunks
        for (pos, chunk) in &self.chunks {
            let mut game_objects = vec![];

            for (id, entity) in self.game_objects_chunks.get(pos).unwrap_or(&HashMap::new()) {
                let (game_object, game_object_type, transform, _, item_drop, _)
                    = query.get(*entity).unwrap();

                let data = match game_object_type {
                    GameObjectType::Debug => GameObjectData::Debug,
                    GameObjectType::ItemDrop => GameObjectData::ItemDrop(item_drop.unwrap().clone()),
                    // Players saved separately
                    GameObjectType::Player => continue
                };

                game_objects.push((*id, game_object.clone(), *transform, data));
            }

            let data = DeserializedChunkData {
                version: 0,
                data: chunk.clone(),
                game_objects,
            };

            let file = File::create(format!(
                "./world/chunks/{:08x}{:08x}{:08x}.chunk",
                pos.x, pos.y, pos.z
            ))
            .unwrap();

            let mut writer = BufWriter::new(file);

            serde_json::to_writer(&mut writer, &data).unwrap();
        }

        fs::write(
            "./world/game_objects",
            GAME_OBJECT_ID_COUNTER.load(Ordering::SeqCst).to_string(),
        )
        .unwrap();

        for (game_object, game_object_type, transform, inventory, _, player_data) in query.iter() {

            let Some(inventory) = inventory else {
                continue
            };

            let Some(player_data) = player_data else {
                continue
            };

            let data = DeserializedPlayerData {
                position: transform.position,
                rotation: transform.rotation,
                inventory: inventory.clone()
            };

            fs::write(
                format!("./world/players/{}", player_data.user_id.0),
                serde_json::to_string(&data).unwrap(),
            )
            .unwrap();
        }
    }
}
