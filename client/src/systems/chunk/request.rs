use crate::game::player::Player;
use crate::systems::chunk::ChunkSystem;
use crate::systems::physics::PhysicsObject;
use bevy::prelude::{EventWriter, Query, ResMut, With};
use nalgebra::Vector3;
use rc_shared::constants::UserId;
use rc_networking::protocol::serverbound::request_chunk::RequestChunk;
use rc_networking::protocol::Protocol;
use rc_networking::types::SendPacket;
use rc_shared::helpers::global_f32_to_local_position;

/// Requests chunks when we move between chunks
pub fn request_chunks(
    player: Query<&PhysicsObject, With<Player>>,
    mut system: ResMut<ChunkSystem>,
    mut chunk_requests: EventWriter<SendPacket>,
) {
    let Ok(object) = player.get_single() else {
        return
    };

    // Get current chunk
    let (current_chunk, _) = global_f32_to_local_position(object.position);
    let (previous_chunk, _) = global_f32_to_local_position(object.previous_position);

    if current_chunk == previous_chunk {
        return;
    }

    #[cfg(target_arch = "wasm32")]
    let render_distance = 6;
    #[cfg(not(target_arch = "wasm32"))]
    let render_distance = 8;

    // Load new chunks
    for x in -render_distance..render_distance {
        for y in -render_distance..render_distance {
            for z in -render_distance..render_distance {
                let potential_chunk = Vector3::new(x, y, z) + current_chunk;

                if (potential_chunk - current_chunk).cast::<f32>().magnitude()
                    > render_distance as f32
                {
                    continue;
                }

                // Try load chunk
                if system.chunks.contains_key(&potential_chunk)
                    || system.requested_chunks.contains(&potential_chunk)
                {
                    continue;
                }

                system.requested_chunks.push(potential_chunk);

                chunk_requests.send(SendPacket(
                    Protocol::RequestChunk(RequestChunk::new(
                        potential_chunk.x,
                        potential_chunk.y,
                        potential_chunk.z,
                    )),
                    UserId(0),
                ));
            }
        }
    }
}
