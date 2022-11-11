use crate::events::connection::ConnectionEvent;
use crate::events::disconnect::DisconnectionEvent;
use crate::systems::authorization::GameUser;
use crate::transport::listener::ServerListener;
use crate::transport::packet::{ReceivePacket, SendPacket};
use crate::TransportSystem;
use bevy_ecs::event::{EventReader, EventWriter};
use bevy_ecs::system::{Res, ResMut};
use bevy_log::{debug, error, info, warn};
use crossbeam::channel::{unbounded, Receiver, Sender};
use rc_client::rc_protocol::constants::{EntityId, UserId};

use rc_client::rc_protocol::protocol::clientbound::ping::Ping;
use rc_client::rc_protocol::protocol::serverbound::pong::Pong;
use rc_client::rc_protocol::protocol::Protocol;

use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const MAX_PING_TIMEOUT_SECONDS: u64 = 10;
const PING_TIME_SECONDS: u64 = 15;

/// Accept connections by users and begin authorisation process
pub fn accept_connections(
    mut system: ResMut<TransportSystem>,
    stream: Res<ServerListener>,
    mut connection_event_writer: EventWriter<ConnectionEvent>,
) {
    // Loop over all new connections
    while let Ok(conn) = stream.receive_connections.recv_timeout(Duration::ZERO) {
        system.total_connections += 1;

        // Generate new userid
        let uid = UserId(system.total_connections as u64);

        info!(
            "Connection request made from {} given UID {:?}",
            conn.0.peer_addr().unwrap(),
            uid
        );

        conn.0.set_nodelay(true).unwrap();

        let (mut read_tcp, mut write_tcp) = conn.0.into_split();

        let (inner_write_packets, read_packets) = unbounded();

        // Read packets
        let read_packet_handle = stream.runtime.spawn(async move {
            loop {
                let mut data = [0; 4]; // 4 Is the size of u32
                match read_tcp.read_exact(&mut data).await {
                    Ok(0) => {
                        warn!("Potentially closed")
                    }
                    Ok(_) => {}
                    Err(e) => {
                        if e.kind() == std::io::ErrorKind::UnexpectedEof {
                            break;
                        }
                        error!("Error reading data from client {:?}", uid);
                        break;
                    }
                };

                let len: u32 = match bincode::deserialize(&data[..]) {
                    Ok(val) => val,
                    Err(e) => {
                        error!("Error reading data from client {:?}: {:?}", uid, e);
                        break;
                    }
                };

                let mut data = vec![0u8; len as usize];

                // Read packet data
                match read_tcp.read_exact(&mut data).await {
                    Ok(val) => val,
                    Err(e) => {
                        if e.kind() == std::io::ErrorKind::UnexpectedEof {
                            break;
                        }
                        error!("Error reading data from client {:?}: {:?}", uid, e);
                        break;
                    }
                };

                let packet: Protocol = match bincode::deserialize(&data[..]) {
                    Ok(val) => val,
                    Err(e) => {
                        error!("Error reading data from client {:?}: {:?}", uid, e);
                        break;
                    }
                };

                // Send packet to receiver
                match inner_write_packets.send(ReceivePacket(packet, uid)) {
                    Ok(_) => {}
                    Err(e) => {
                        // Channel disconnected, delete this task
                        debug!(
                            "Failed to read packet for user {:?} destroyed: {:?}",
                            uid, e
                        );
                        break;
                    }
                }
            }
        });

        let (write_packets, inner_read_packets): (Sender<SendPacket>, Receiver<SendPacket>) =
            unbounded();

        // Write packets
        let write_packet_handle = stream.runtime.spawn(async move {
            while let Ok(packet) = inner_read_packets.recv() {
                // Write
                let packet = match bincode::serialize(&packet.0) {
                    Ok(val) => val,
                    Err(e) => {
                        error!("Error reading data from client {:?}: {:?}", uid, e);
                        break;
                    }
                };
                // Just size for now
                let header: u32 = packet.len() as u32;

                if let Err(e) = write_tcp
                    .write_all(&bincode::serialize(&header).unwrap())
                    .await
                {
                    debug!("Failed to write packet for user {:?}: {:?}", uid, e);
                    break;
                }
                if let Err(e) = write_tcp.write_all(&packet).await {
                    debug!("Failed to write packet for user {:?}: {:?}", uid, e);
                    break;
                }

                if let Err(e) = write_tcp.flush().await {
                    debug!("Failed to flush packet for user {:?}: {:?}", uid, e);
                    break;
                }
            }
        });

        // Start reading data from socket
        let user = GameUser {
            name: None,
            read_packets,
            write_packets,
            read_packet_handle,
            write_packet_handle,
            last_ping: Ping::new(),
            last_pong: Pong::new(),
            user_id: uid,
            entity_id: EntityId(uid.0),
            disconnected: false,
        };

        system.clients.insert(uid, user);

        // Send connection event
        connection_event_writer.send(ConnectionEvent::new(uid));
    }
}

pub fn send_packets(mut system: ResMut<TransportSystem>, mut packets: EventReader<SendPacket>) {
    for packet in packets.iter() {
        debug!("<- {:?}", packet.0);
        if let Some(mut user) = system.clients.get_mut(&packet.1) {
            let packet = packet.clone();
            match user.write_packets.send(packet) {
                Ok(_) => {}
                Err(e) => {
                    error!(
                        "Unable to communicate with client {:?}'s thread: {:?}",
                        user.user_id, e
                    );
                    user.disconnected = true;
                }
            }
        }
    }
}

pub fn receive_packets(system: ResMut<TransportSystem>, mut packets: EventWriter<ReceivePacket>) {
    for (_, user) in &system.clients {
        while let Ok(packet) = user.read_packets.recv_timeout(Duration::ZERO) {
            debug!("-> {:?}", packet.0);
            packets.send(packet);
        }
    }
}

pub fn prune_users(
    mut system: ResMut<TransportSystem>,
    mut event: EventWriter<DisconnectionEvent>,
) {
    let mut delete_users = Vec::new();
    for (uid, user) in &system.clients {
        if user.disconnected {
            delete_users.push(*uid);
        }
        if user.read_packet_handle.is_finished() || user.write_packet_handle.is_finished() {
            delete_users.push(*uid);
            debug!(
                "Detected writing or reading thread for {:?} finished. Terminating connection.",
                uid
            );
        }
    }

    for client in delete_users {
        if let Some(user) = system.clients.remove(&client) {
            info!("Disconnected user {:?}", client);
            event.send(DisconnectionEvent { client, user });
        } else {
            info!("Disconnected user ???");
        }
    }
}

/// Sends ping requests to check if the server is still connected
pub fn check_connections(
    mut system: ResMut<TransportSystem>,
    mut pong_responses: EventReader<ReceivePacket>,
    mut ping_requests: EventWriter<SendPacket>,
) {
    for (uid, mut stream) in &mut system.clients {
        // If ping hasn't been sent in the last PING_TIME_SECONDS, then send it
        if Ping::new().code - stream.last_ping.code > PING_TIME_SECONDS {
            // Send new ping request
            ping_requests.send(SendPacket(Protocol::Ping(Ping::new()), *uid));
            stream.last_ping = Ping::new();
        }

        // If the last received ping was over the timeout seconds ago then disconnect user
        if Ping::new().code - stream.last_ping.code > PING_TIME_SECONDS + MAX_PING_TIMEOUT_SECONDS {
            // Disconnect for not responding to pings
            info!("Disconnected Client {:?}: Timed Out", uid);
            stream.disconnected = true;
        }
    }

    // Loop over network events to check for ping events
    for req in pong_responses.iter() {
        if let ReceivePacket(Protocol::Pong(req), user) = req {
            system.clients.get_mut(user).unwrap().last_pong = *req;
        }
    }
}
