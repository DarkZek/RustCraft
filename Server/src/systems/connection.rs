use bevy_ecs::event::EventReader;
use bevy_ecs::system::ResMut;
use bevy_log::info;
use bevy_testing_protocol::channels::Channels;
use bevy_testing_protocol::protocol::{Protocol};
use bevy_testing_protocol::protocol::clientbound::chunk_update::PartialChunkUpdate;
use bevy_testing_protocol::protocol::clientbound::player_join::PlayerJoin;
use naia_bevy_server::events::ConnectionEvent;
use naia_bevy_server::Server;
use naia_bevy_server::shared::Random;
use nalgebra::{Vector2, Vector3};
use crate::game::chunk::ChunkData;
use crate::game::player::Player;
use crate::resources::Global;

pub fn connection_event<'world, 'state>(
    mut event_reader: EventReader<ConnectionEvent>,
    mut global: ResMut<Global>,
    mut server: Server<'world, 'state, Protocol, Channels>,
) {
    for event in event_reader.iter() {
        let ConnectionEvent(user_key) = event;

        let address = server
            .user_mut(user_key)
            // Add User to the main Room
            .enter_room(&global.main_room_key)
            // Get User's address for logging
            .address();

        info!("Connection request from: {}", address);

        // Create components for Entity to represent new player

        // Spawn entity
        let entity = server
            // Spawn new Square Entity
            .spawn()
            // Add Entity to main Room
            .enter_room(&global.main_room_key)
            // Insert Position component
            //.insert(position)
            // return Entity id
            .id();

        let name = global.authentication_requests.remove(&user_key).unwrap();

        let mut packet = PlayerJoin::new(&name);

        packet.entity.set(&server, &entity);

        for (other_user, other_user_player) in &global.user_to_prediction_map {
            // Notify all other players of the new player
            server.send_message(other_user, Channels::StatusUpdate, &packet);

            // Notify new player of all other players
            let mut packet = PlayerJoin::new(&other_user_player.name);
            packet.entity.set(&server, &other_user_player.entity);

            // Sending user player join packets
            info!("Sending player join packet to player");
            server.send_message(user_key, Channels::StatusUpdate, &packet);
        }

        global.user_to_prediction_map.insert(*user_key, Player::new(entity, name));

        // Send chunks
        for (_, chunk) in &global.chunks {
            chunk.send(&mut server, &user_key);
        }
    }
}
