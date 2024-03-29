use crate::systems::chunk::ChunkSystem;
use crate::{EventWriter, PlayerSpawnEvent, TransportSystem};
use bevy::prelude::{Res, ResMut};
use rc_networking::protocol::clientbound::update_loading::UpdateLoading;
use rc_networking::protocol::Protocol;
use rc_networking::types::SendPacket;

/// This system sends the close loading screen packet once users have successfully
pub fn detect_finish_join(
    mut transport: ResMut<TransportSystem>,
    chunk_system: Res<ChunkSystem>,
    mut event_writer: EventWriter<SendPacket>,
    mut join_writer: EventWriter<PlayerSpawnEvent>,
) {
    let mut remove_clients = Vec::new();

    for user_id in &transport.initialising_clients {
        // If they have no chunks requested, they're ready
        if let Some(finished) = chunk_system
            .requesting_chunks
            .get(&user_id)
            .map(|v| v.len() == 0)
        {
            if !finished {
                continue;
            }
            // Send stop loading packet
            event_writer.send(SendPacket(
                Protocol::UpdateLoading(UpdateLoading::new(false)),
                *user_id,
            ));
            remove_clients.push(*user_id);

            join_writer.send(PlayerSpawnEvent { id: *user_id });
        }
    }

    for client in remove_clients {
        transport.initialising_clients.remove(&client);
    }
}
