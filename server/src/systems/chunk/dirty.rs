use std::ops::AddAssign;
use bevy::prelude::{EventWriter, Res, ResMut};
use rc_networking::protocol::clientbound::chunk_update::FullChunkUpdate;
use rc_networking::protocol::Protocol;
use rc_networking::types::SendPacket;
use crate::game::world::data::WorldData;
use crate::systems::chunk::ChunkSystem;

pub fn sync_dirty_chunks(
    mut world_data: ResMut<WorldData>,
    mut chunk_system: ResMut<ChunkSystem>,
    mut send_packet: EventWriter<SendPacket>
) {

    let ChunkSystem {
        user_loaded_chunks,
        chunk_outstanding_requests,
        ..
    } = &mut *chunk_system;

    world_data
        .chunks
        .iter_mut()
        .filter(|(_, data)| data.dirty)
        .for_each(|(pos, data)| {

            // Send update to all users
            for (user, loaded_chunks) in user_loaded_chunks.iter_mut() {
                // TODO: Check outstanding requests first to not overwhealm user

                if loaded_chunks.contains(pos) {
                    // Send update
                    send_packet.send(SendPacket(
                        Protocol::FullChunkUpdate(FullChunkUpdate {
                            data: data.world.clone(),
                            x: data.position.x,
                            y: data.position.y,
                            z: data.position.z,
                        }),
                        *user
                    ));

                    chunk_outstanding_requests
                        .get_mut(user)
                        .unwrap()
                        .add_assign(1);
                }
            }

            data.dirty = false;
        });
}