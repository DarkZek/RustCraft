use crate::game::game_object::GameObject;
use crate::game::transform::Transform;
use crate::{TransportSystem, WorldData};
use bevy::prelude::{Commands, Entity, Event, EventReader, EventWriter, Res, ResMut};

use rc_shared::constants::GameObjectId;
use rc_networking::protocol::clientbound::spawn_game_object::SpawnGameObject;
use rc_networking::protocol::Protocol;
use rc_networking::types::SendPacket;
use rc_shared::game_objects::GameObjectData;
use rc_shared::helpers::global_f32_to_local_position;

#[derive(Event)]
pub struct SpawnGameObjectEvent {
    pub entity_id: Entity,
    pub id: GameObjectId,
    pub data: GameObjectData,
}

#[derive(Event)]
pub struct SpawnGameObjectRequest {
    pub transform: Transform,
    pub data: GameObjectData,
    pub entity: Option<Entity>,
    pub id: GameObjectId
}

/// Spawns entities requested
pub fn spawn_entities(
    mut events: EventReader<SpawnGameObjectRequest>,
    mut command: Commands,
    mut event_writer: EventWriter<SpawnGameObjectEvent>,
    mut packet_writer: EventWriter<SendPacket>,
    users: Res<TransportSystem>,
    mut global: ResMut<WorldData>,
) {
    for event in events.read() {
        let entity = if let Some(entity) = event.entity {
            entity
        } else {
            command.spawn_empty().id()
        };

        let entity = command
            .entity(entity)
            .insert(Transform::from_translation(event.transform.position))
            .insert(GameObject {
                data: event.data.clone(),
                id: event.id,
            })
            .id();

        let id = event
            .id;

        event_writer.send(SpawnGameObjectEvent {
            entity_id: entity.clone(),
            data: event.data.clone(),
            id,
        });

        let (chunk_pos, _) = global_f32_to_local_position(event.transform.position);

        global.insert_game_object(id, entity, chunk_pos);

        for (user, _) in &users.clients {
            packet_writer.send(SendPacket(
                Protocol::SpawnGameObject(SpawnGameObject {
                    id,
                    loc: [
                        event.transform.position.x,
                        event.transform.position.y,
                        event.transform.position.z,
                    ],
                    rot: [
                        event.transform.rotation.as_vector().x,
                        event.transform.rotation.as_vector().y,
                        event.transform.rotation.as_vector().z,
                        event.transform.rotation.as_vector().w,
                    ],
                    data: event.data.clone(),
                }),
                *user,
            ));
        }
    }
}
