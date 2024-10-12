use bevy::prelude::*;
use rc_networking::protocol::Protocol;
use rc_networking::types::ReceivePacket;
use crate::systems::chunk::ChunkSystem;

pub fn receive_column_updates(
    mut events: EventReader<ReceivePacket>,
    mut data: ResMut<ChunkSystem>
) {
    for packet in events.read() {
        let Protocol::ChunkColumnUpdate(update) = packet.0 else {
            continue
        };

        data.chunk_columns.insert(update.position, update.data.clone());

        // TODO: Rebuild chunks?
    }
}