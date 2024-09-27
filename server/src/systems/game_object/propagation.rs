use crate::game::game_object::GameObject;
use crate::game::transform::Transform;
use crate::{EventWriter, PlayerSpawnEvent, WorldData};
use bevy::log::warn;
use bevy::prelude::{EventReader, Query, ResMut};
use rc_networking::protocol::clientbound::spawn_game_object::SpawnGameObject;
use rc_networking::protocol::Protocol;
use rc_networking::types::SendPacket;
use rc_shared::game_objects::{GameObjectData, GameObjectType, ItemDropGameObjectData, PlayerGameObjectData};

pub fn propagate_game_objects_to_new_clients(
    mut events: EventReader<PlayerSpawnEvent>,
    global: ResMut<WorldData>,
    game_object_data: Query<(&Transform, &GameObject, &GameObjectType, Option<&PlayerGameObjectData>, Option<&ItemDropGameObjectData>)>,
    mut send_packet: EventWriter<SendPacket>,
) {
    for event in events.read() {

        // Spawn other entities for new player
        for (id, entity) in &global.game_objects_mapping {
            if let Ok((transform, game_object, game_object_type, player_data, item_drop)) = game_object_data.get(*entity) {

                let data = match game_object_type {
                    GameObjectType::Debug => GameObjectData::Debug,
                    GameObjectType::ItemDrop => GameObjectData::ItemDrop(item_drop.unwrap().clone()),
                    GameObjectType::Player => GameObjectData::Player(player_data.unwrap().clone())
                };

                send_packet.send(SendPacket(
                    Protocol::SpawnGameObject(SpawnGameObject {
                        id: *id,
                        loc: [
                            transform.position.x,
                            transform.position.y,
                            transform.position.z,
                        ],
                        rot: transform.rotation.coords.into(),
                        data,
                    }),
                    event.id,
                ));
            } else {
                warn!("Tried to spawn object that does not exist");
            }
        }
    }
}
