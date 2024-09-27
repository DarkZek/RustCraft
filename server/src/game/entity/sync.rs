use bevy::prelude::{Commands, Entity, EventWriter, Or, Query, Res, With};
use nalgebra::Vector3;
use rc_networking::protocol::clientbound::game_object_moved::GameObjectMoved;
use rc_networking::protocol::clientbound::game_object_rotated::GameObjectRotated;
use rc_networking::protocol::Protocol;
use rc_networking::types::SendPacket;
use rc_shared::game_objects::PlayerGameObjectData;
use crate::game::entity::{DirtyPosition, DirtyRotation};
use crate::game::game_object::GameObject;
use crate::game::transform::Transform;
use crate::helpers::global_to_local_position;
use crate::systems::chunk::ChunkSystem;

pub fn sync_entities(
    transform_query: Query<(&Transform, &PlayerGameObjectData, &GameObject)>,
    entities_query: Query<(Entity, &Transform, &GameObject, Option<&DirtyPosition>, Option<&DirtyRotation>), Or<(With<DirtyPosition>, With<DirtyRotation>)>>,
    chunk_system: Res<ChunkSystem>,
    mut ew: EventWriter<SendPacket>,
    mut commands: Commands
) {
    for (
        entity,
        transform,
        game_object,
        position,
        rotation
    ) in entities_query.iter() {

        // Get the chunk the entity is located within
        let (chunk_pos, local_pos) = global_to_local_position(
            Vector3::new(
                transform.position.x as i32,
                transform.position.y as i32,
                transform.position.z as i32
            )
        );

        // Loop through players
        for (player_transform, player_data, player_game_object) in transform_query.iter() {

            // If player has chunk loaded
            if !chunk_system.user_loaded_chunks.get(&player_data.user_id).unwrap().contains(&chunk_pos) {
                continue
            }

            if game_object.id == player_game_object.id {
                // Don't send player updates for their own player
                // Players currently have full authority over their position
                continue
            }

            // Send updates
            if position.is_some() {
                ew.send(SendPacket(
                    Protocol::GameObjectMoved(
                        GameObjectMoved {
                            entity: game_object.id,
                            x: transform.position.x,
                            y: transform.position.y,
                            z: transform.position.z,
                        }
                    ),
                    player_data.user_id
                ));
                commands.entity(entity).remove::<DirtyPosition>();
            }
            if rotation.is_some() {
                ew.send(SendPacket(
                    Protocol::GameObjectRotated(
                        GameObjectRotated {
                            entity: game_object.id,
                            x: transform.rotation.coords.x,
                            y: transform.rotation.coords.y,
                            z: transform.rotation.coords.z,
                            w: transform.rotation.coords.w,
                        }
                    ),
                    player_data.user_id
                ));
                commands.entity(entity).remove::<DirtyRotation>();
            }
        }
    }
}