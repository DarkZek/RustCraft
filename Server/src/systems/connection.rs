use bevy_ecs::event::{EventReader, EventWriter};
use bevy_ecs::system::ResMut;
use bevy_log::info;
use nalgebra::{Vector2, Vector3};
use crate::events::authorization::AuthorizationEvent;
use crate::events::connection::ConnectionEvent;
use crate::game::chunk::ChunkData;
use crate::game::player::Player;
use crate::resources::World;

pub fn connection_event<'world, 'state>(
    mut event_reader: EventReader<ConnectionEvent>,
    mut event_writer: EventWriter<AuthorizationEvent>
) {
    for connection in event_reader.iter() {
        event_writer.send(AuthorizationEvent::new(connection.user));
    }

    // TODO: Add authorisation. For now just go straight to allow authorisation
    // for event in event_reader.iter() {
    //     let ConnectionEvent(user_key) = event;
    //
    //     let address = server
    //         .user_mut(user_key)
    //         // Add User to the main Room
    //         .enter_room(&global.main_room_key)
    //         // Get User's address for logging
    //         .address();
    //
    //     info!("Connection request from: {}", address);
    //
    //     // Create components for Entity to represent new player
    //
    //     // Spawn entity
    //     let entity = server
    //         // Spawn new Square Entity
    //         .spawn()
    //         // Add Entity to main Room
    //         .enter_room(&global.main_room_key)
    //         // Insert Position component
    //         //.insert(position)
    //         // return Entity id
    //         .id();
    //
    //     let name = global.authentication_requests.remove(&user_key).unwrap();
    //
    //     let mut packet = PlayerJoin::new(&name);
    //
    //     packet.entity.set(&server, &entity);
    //
    //     for (other_user, other_user_player) in &global.user_to_prediction_map {
    //         // Notify all other players of the new player
    //         server.send_message(other_user, Channels::StatusUpdate, &packet);
    //
    //         // Notify new player of all other players
    //         let mut packet = PlayerJoin::new(&other_user_player.name);
    //         packet.entity.set(&server, &other_user_player.entity);
    //
    //         // Sending user player join packets
    //         info!("Sending player join packet to player");
    //         server.send_message(user_key, Channels::StatusUpdate, &packet);
    //     }
    //
    //     global.user_to_prediction_map.insert(*user_key, Player::new(entity, name));
    //
    //     // Send chunks
    //     for (_, chunk) in &global.chunks {
    //         chunk.send(&mut server, &user_key);
    //     }
    // }
}
