use crate::events::connection::ConnectionEvent;
use crate::events::disconnect::DisconnectionEvent;
use crate::systems::authorization::GameUser;
use crate::TransportSystem;
use bevy_ecs::event::{EventReader, EventWriter};
use bevy_ecs::system::{ResMut};
use bevy_log::{debug};
use rc_client::rc_protocol::constants::{EntityId};

use rc_client::rc_protocol::protocol::clientbound::ping::Ping;
use rc_client::rc_protocol::protocol::serverbound::pong::Pong;



use rc_client::rc_networking::server::ServerSocket;
use rc_client::rc_protocol::types::{ReceivePacket, SendPacket};

const MAX_PING_TIMEOUT_SECONDS: u64 = 10;
const PING_TIME_SECONDS: u64 = 15;

/// Accept connections by users and begin authorisation process
pub fn accept_connections(
    mut system: ResMut<TransportSystem>,
    mut socket: ResMut<ServerSocket>,
    mut connection_event_writer: EventWriter<ConnectionEvent>,
    mut send_packets: EventReader<SendPacket>,
    mut recieve_packets: EventWriter<ReceivePacket>,
) {

    let results = socket.poll();

    // Loop over all new connections
    for conn in results.connections {
        // Start reading data from socket
        let user = GameUser {
            name: None,
            last_ping: Ping::new(),
            last_pong: Pong::new(),
            user_id: conn.user,
            entity_id: EntityId(conn.user.0),
            disconnected: false,
        };

        system.clients.insert(conn.user, user);

        // Send connection event
        connection_event_writer.send(ConnectionEvent::new(conn.user));
    }

    // Put new packets into ECS
    for packet in results.packets {
        debug!("-> {:?}", packet.0);
        recieve_packets.send(packet);
    }

    for packet in send_packets.iter() {
        socket.send_packet(packet.clone());
    }
}

pub fn prune_users(
    _system: ResMut<TransportSystem>,
    _event: EventWriter<DisconnectionEvent>,
) {
    // let mut delete_users = Vec::new();
    // for (uid, user) in &system.clients {
    //     if user.disconnected {
    //         delete_users.push(*uid);
    //     }
    //     if user.read_packet_handle.is_finished() || user.write_packet_handle.is_finished() {
    //         delete_users.push(*uid);
    //         debug!(
    //             "Detected writing or reading thread for {:?} finished. Terminating connection.",
    //             uid
    //         );
    //     }
    // }
    //
    // for client in delete_users {
    //     if let Some(user) = system.clients.remove(&client) {
    //         info!("Disconnected user {:?}", client);
    //         event.send(DisconnectionEvent { client, user });
    //     } else {
    //         info!("Disconnected user ???");
    //     }
    // }
}

/// Sends ping requests to check if the server is still connected
pub fn check_connections(
    _system: ResMut<TransportSystem>,
    _pong_responses: EventReader<ReceivePacket>,
    _ping_requests: EventWriter<SendPacket>,
) {
    // for (uid, mut stream) in &mut system.clients {
    //     // If ping hasn't been sent in the last PING_TIME_SECONDS, then send it
    //     if Ping::new().code - stream.last_ping.code > PING_TIME_SECONDS {
    //         // Send new ping request
    //         ping_requests.send(SendPacket(Protocol::Ping(Ping::new()), *uid));
    //         stream.last_ping = Ping::new();
    //     }
    //
    //     // If the last received ping was over the timeout seconds ago then disconnect user
    //     if Ping::new().code - stream.last_ping.code > PING_TIME_SECONDS + MAX_PING_TIMEOUT_SECONDS {
    //         // Disconnect for not responding to pings
    //         info!("Disconnected Client {:?}: Timed Out", uid);
    //         stream.disconnected = true;
    //     }
    // }
    //
    // // Loop over network events to check for ping events
    // for req in pong_responses.iter() {
    //     if let ReceivePacket(Protocol::Pong(req), user) = req {
    //         system.clients.get_mut(user).unwrap().last_pong = *req;
    //     }
    // }
}
