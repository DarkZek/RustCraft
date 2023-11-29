use crate::systems::chunk::ChunkSystem;
use crate::{EventWriter, TransportSystem};
use bevy::prelude::{Res, ResMut};
use rc_networking::protocol::clientbound::update_loading::UpdateLoading;
use rc_networking::protocol::Protocol;
use rc_networking::types::SendPacket;

/// This system sends the close loading screen packet once users have successfully
pub fn detect_finish_join(
    mut transport: ResMut<TransportSystem>,
    chunk_system: Res<ChunkSystem>,
    mut event_writer: EventWriter<SendPacket>,
) {
    let mut remove_clients = Vec::new();

    for user_id in &transport.initialising_clients {
        // If they have no chunks requested, they're ready
        if chunk_system
            .requesting_chunks
            .get(&user_id)
            .map(|v| v.len() == 0)
            .unwrap()
        {
            // Send stop loading packet
            event_writer.send(SendPacket(
                Protocol::UpdateLoading(UpdateLoading::new(false)),
                *user_id,
            ));
            remove_clients.push(*user_id);
        }
    }

    for client in remove_clients {
        transport.initialising_clients.remove(&client);
    }
}
