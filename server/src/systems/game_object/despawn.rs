use bevy::prelude::*;
use nalgebra::Vector3;
use rc_networking::protocol::Protocol;
use rc_networking::types::SendPacket;
use crate::game::game_object::GameObject;
use crate::helpers::global_to_local_position;
use crate::systems::chunk::ChunkSystem;
use crate::transport::TransportSystem;

#[derive(Component)]
pub struct DespawnGameObject;

pub fn despawn_game_objects(
    query: Query<(Entity, &crate::game::transform::Transform, &GameObject), With<DespawnGameObject>>,
    chunk_system: Res<ChunkSystem>,
    system: Res<TransportSystem>,
    mut commands: Commands,
    mut send_packet: EventWriter<SendPacket>
) {

    for (entity, transform, game_object) in query.iter() {

        let (chunk_pos, _) = global_to_local_position(
            Vector3::new(
                transform.position.x.round() as i32,
                transform.position.y.round() as i32,
                transform.position.z.round() as i32
            )
        );

        for (user_id, user) in &system.clients {
            // If player has chunk loaded
            if !chunk_system.user_loaded_chunks.get(user_id).unwrap().contains(&chunk_pos) {
                continue
            }

            info!("Sending despawn notification");

            send_packet.send(
                SendPacket(
                    Protocol::DespawnGameObject(
                        rc_networking::protocol::clientbound::despawn_game_object::DespawnGameObject::new(game_object.id)
                    ),
                    *user_id
                )
            );
        }

        commands.entity(entity).despawn_recursive();
    }
}