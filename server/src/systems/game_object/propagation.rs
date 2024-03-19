use crate::game::game_object::GameObject;
use crate::game::transform::Transform;
use crate::{EventWriter, PlayerSpawnEvent, WorldData};
use bevy::prelude::{EventReader, Query, ResMut};
use rc_networking::protocol::clientbound::spawn_game_object::SpawnGameObject;
use rc_networking::protocol::Protocol;
use rc_networking::types::SendPacket;

pub fn propagate_game_objects_to_new_clients(
    mut events: EventReader<PlayerSpawnEvent>,
    global: ResMut<WorldData>,
    game_object_data: Query<(&Transform, &GameObject)>,
    mut send_packet: EventWriter<SendPacket>,
) {
    for event in events.read() {
        // Spawn other entities for new player
        for (id, entity) in &global.game_objects_mapping {
            let (transform, game_object) = game_object_data.get(*entity).unwrap();
            send_packet.send(SendPacket(
                Protocol::SpawnGameObject(SpawnGameObject {
                    id: *id,
                    loc: [
                        transform.position.x,
                        transform.position.y,
                        transform.position.z,
                    ],
                    rot: transform.rotation.coords.into(),
                    data: game_object.data.clone(),
                }),
                event.id,
            ));
        }
    }
}
