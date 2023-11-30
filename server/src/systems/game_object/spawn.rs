use crate::game::game_object::GameObject;
use crate::game::transform::Transform;
use crate::game::world::data::GAME_OBJECT_ID_COUNTER;
use crate::{TransportSystem, WorldData};
use bevy::prelude::{Commands, Entity, Event, EventReader, EventWriter, Res, ResMut};

use rc_networking::constants::GameObjectId;
use rc_networking::protocol::clientbound::spawn_game_object::SpawnGameObject;
use rc_networking::protocol::Protocol;
use rc_networking::types::SendPacket;
use rc_shared::helpers::global_f32_to_local_position;
use std::sync::atomic::Ordering;

#[derive(Event)]
pub struct SpawnGameObjectEvent {
    pub entity_id: Entity,
    pub id: GameObjectId,
    pub object_type: u32,
}

#[derive(Event)]
pub struct SpawnGameObjectRequest {
    pub transform: Transform,
    pub object_type: u32,
    pub id: Option<GameObjectId>,
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
        let entity = command
            .spawn(Transform::from_translation(event.transform.position))
            .insert(GameObject {
                object_type: event.object_type,
            })
            .id();

        let id = event
            .id
            .unwrap_or_else(|| GameObjectId(GAME_OBJECT_ID_COUNTER.fetch_add(1, Ordering::SeqCst)));

        event_writer.send(SpawnGameObjectEvent {
            entity_id: entity.clone(),
            object_type: event.object_type,
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
                    object_type: event.object_type,
                }),
                *user,
            ));
        }
    }
}
