use crate::game::game_object::GameObject;
use crate::game::transform::Transform;
use crate::game::world::data::ENTITY_ID_COUNT;
use crate::{TransportSystem, WorldData};
use bevy::prelude::{Commands, Entity, Event, EventReader, EventWriter, Res, ResMut};
use nalgebra::Vector3;
use rc_networking::constants::GameObjectId;
use rc_networking::protocol::clientbound::spawn_game_object::SpawnGameObject;
use rc_networking::protocol::Protocol;
use rc_networking::types::SendPacket;
use std::sync::atomic::Ordering;

#[derive(Event)]
pub struct SpawnGameObjectEvent {
    pub entity_id: Entity,
    pub id: GameObjectId,
    pub object_type: u32,
}

#[derive(Event)]
pub struct SpawnGameObjectRequest {
    pub position: Vector3<f32>,
    pub object_type: u32,
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
            .spawn(Transform::from_translation(event.position))
            .insert(GameObject)
            .id();

        let id = GameObjectId(ENTITY_ID_COUNT.fetch_add(1, Ordering::Acquire));

        event_writer.send(SpawnGameObjectEvent {
            entity_id: entity.clone(),
            object_type: event.object_type,
            id,
        });

        global.entities.insert(id, entity);

        for (user, _) in &users.clients {
            packet_writer.send(SendPacket(
                Protocol::SpawnGameObject(SpawnGameObject {
                    id,
                    loc: [event.position.x, event.position.y, event.position.z],
                    rot: [0.0, 0.0, 0.0, 0.0],
                    object_type: event.object_type,
                }),
                *user,
            ));
        }
    }
}
