use bevy_ecs::event::EventReader;
use bevy_ecs::prelude::Res;
use bevy_ecs::system::ResMut;
use bevy_testing_protocol::channels::Channels;
use bevy_testing_protocol::constants::CHUNK_SIZE;
use bevy_testing_protocol::protocol::clientbound::block_update::BlockUpdate;
use bevy_testing_protocol::protocol::Protocol;
use bevy_testing_protocol::protocol::clientbound::player_rotated::PlayerRotated;
use bevy_testing_protocol::protocol::clientbound::player_moved::PlayerMoved;
use naia_bevy_server::events::MessageEvent;
use naia_bevy_server::Server;
use nalgebra::Vector3;
use crate::game::chunk::ChunkData;
use crate::helpers::global_to_local_position;
use crate::info;
use crate::resources::Global;

pub fn receive_message_event(
    mut event_reader: EventReader<MessageEvent<Protocol, Channels>>,
    mut server: Server<Protocol, Channels>,
    mut global: ResMut<Global>
) {
    for event in event_reader.iter() {
        match event {
            MessageEvent(user_key, Channels::PlayerCommand, Protocol::PlayerMove(packet)) => {
                // Update all other clients
                let entity = global.user_to_prediction_map.get(&user_key).unwrap();

                // TODO: Don't trust user input

                let mut packet = PlayerMoved::new(*packet.x, *packet.y, *packet.z);

                packet.player.set(&server, &entity.entity);

                for (client, _) in &global.user_to_prediction_map {
                    if *client == *user_key {
                        continue;
                    }
                    //info!("Move packet sent to {:?}", client);
                    server.send_message(client, Channels::StatusUpdate, &packet);
                }
            }
            MessageEvent(user_key, Channels::PlayerCommand, Protocol::PlayerRotate(packet)) => {
                // Update all other clients
                let entity = global.user_to_prediction_map.get(&user_key).unwrap();

                // TODO: Don't trust user input

                let mut packet = PlayerRotated::new(*packet.x, *packet.y, *packet.z, *packet.w);

                packet.player.set(&server, &entity.entity);

                for (client, _) in &global.user_to_prediction_map {
                    if *client == *user_key {
                        continue;
                    }
                    server.send_message(client, Channels::StatusUpdate, &packet);
                }
            }
            MessageEvent(user_key, Channels::PlayerCommand, Protocol::BlockUpdate(packet)) => {
                // TODO: Don't trust user input

                let mut packet = BlockUpdate::new(*packet.id, *packet.x, *packet.y, *packet.z);

                for (client, _) in &global.user_to_prediction_map {
                    if *client == *user_key {
                        continue;
                    }
                    server.send_message(client, Channels::ChunkUpdates, &packet);
                }

                let (chunk_loc, inner_loc) = global_to_local_position(Vector3::new(*packet.x, *packet.y, *packet.z));

                // Store
                if let Some(mut chunk) = global.chunks.get_mut(&chunk_loc) {
                    // Found chunk! Update block
                    chunk.world[inner_loc.x][inner_loc.y][inner_loc.z] = 1;
                } else {
                    // Create chunk data
                    let mut chunk = [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

                    // Set block
                    chunk[inner_loc.x][inner_loc.y][inner_loc.z] = 1;

                    // Create chunk
                    global.chunks.insert(chunk_loc, ChunkData::new(chunk_loc, chunk));
                }

            }
            _ => {}
        }
    }
}