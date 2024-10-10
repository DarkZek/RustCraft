use bevy::prelude::{EventWriter, Res, ResMut};
use nalgebra::Vector2;
use rc_networking::protocol::clientbound::chunk_column_update::ChunkColumnUpdate;
use rc_networking::protocol::Protocol;
use rc_networking::types::SendPacket;
use crate::game::world::data::WorldData;
use crate::systems::chunk::ChunkSystem;

pub mod update;

pub fn propagate_chunk_columns(
    mut world_data: ResMut<WorldData>,
    chunk_system: Res<ChunkSystem>,
    mut send_packet: EventWriter<SendPacket>
) {
    // Get dirty chunks
    world_data
        .chunks_columns
        .iter_mut()
        .filter(|(_, data)| data.dirty)
        .for_each(|(pos, data)| {
            for (user_id, player_loaded_columns) in &chunk_system.user_loaded_columns {
                if !player_loaded_columns.contains(pos) {
                    // Player doesn't have chunk from this column loaded
                    continue
                }

                send_packet.send(SendPacket(
                    Protocol::ChunkColumnUpdate(ChunkColumnUpdate {
                        position: *pos,
                        data: data.clone(),
                    }),
                    *user_id
                ));
            }

            data.dirty = false;
        });
}