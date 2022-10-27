use crate::events::connection::ConnectionEvent;
use crate::systems::authorization::GameUser;
use crate::transport::listener::ServerListener;
use crate::transport::packet::{ReceivePacket, SendPacket};
use crate::TransportSystem;
use bevy_ecs::event::{EventReader, EventWriter};
use bevy_ecs::system::{Res, ResMut};
use bevy_log::{debug, error, info, warn};
use mio::{Events, Interest, Token};
use rustcraft_protocol::constants::UserId;
use rustcraft_protocol::error::ProtocolError;
use rustcraft_protocol::protocol::clientbound::ping::Ping;
use rustcraft_protocol::protocol::Protocol;
use std::io;
use std::io::Write;
use std::net::Shutdown;
use std::time::{Duration, Instant, SystemTime};

const MAX_PING_TIMEOUT_SECONDS: u64 = 10;
const PING_TIME_SECONDS: u64 = 15;

pub const SERVER: Token = Token(0);

/// Accept connections by users and begin authorisation process
pub fn accept_connections(
    mut system: ResMut<TransportSystem>,
    mut stream: ResMut<ServerListener>,
    mut connection_event_writer: EventWriter<ConnectionEvent>,
    mut packet_event_writer: EventWriter<ReceivePacket>,
) {
    // Remove all disconnected clients
    let mut disconnected_clients = Vec::new();
    for (token, client) in system.clients.iter() {
        if client.disconnected {
            disconnected_clients.push(token.clone());
        }
    }

    // Remove clients from mio listener
    for token in disconnected_clients {
        let mut client = system.clients.remove(&token).unwrap();

        stream
            .poll
            .registry()
            .deregister(&mut client.stream.stream)
            .unwrap();
    }

    let mut events = Events::with_capacity(128);

    // Poll for new events
    stream.poll.poll(&mut events, Some(Duration::ZERO)).unwrap();

    for event in events.iter() {
        match event.token() {
            SERVER => loop {
                // Connection request being made!
                let (mut connection, address) = match stream.stream().accept() {
                    Ok((connection, address)) => (connection, address),
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                        // If we get a `WouldBlock` error we know our
                        // listener has no more incoming connections queued,
                        // so we can return to polling and wait for some
                        // more.
                        break;
                    }
                    Err(e) => {
                        // If it was any other kind of error, something went
                        // wrong and we terminate with an error.
                        warn!("Failed to accept connection {}", e);
                        continue;
                    }
                };

                system.total_connections += 1;

                // Generate new userid
                let uid = UserId(system.total_connections as u64);

                info!(
                    "Connection request made from {} given UID {:?}",
                    address, uid
                );

                // Give new connection an id for polling
                stream
                    .poll
                    .registry()
                    .register(
                        &mut connection,
                        Token(system.total_connections),
                        Interest::READABLE.add(Interest::WRITABLE),
                    )
                    .unwrap();

                // Create a new user and record it
                let mut user = GameUser::new(connection);
                system.clients.insert(uid, user);

                // Send connection event
                connection_event_writer.send(ConnectionEvent::new(uid));
            },
            token => {
                let id = UserId(token.0 as u64);
                // Read packets
                let mut user = system.clients.get_mut(&id).unwrap();

                let mut client_disconnect = false;

                if event.is_readable() {
                    loop {
                        // Verify there is data to read
                        let mut data = vec![0u8; 4];
                        match stream.stream.as_mut().unwrap().stream.peek(&mut data) {
                            Ok(_) => {}
                            Err(_) => {
                                break;
                            }
                        }

                        match user.stream.read_packet() {
                            Ok(n) => {
                                debug!("-> {:?}", n);
                                packet_event_writer.send(ReceivePacket(n, id));
                            }
                            // Would block "errors" are the OS's way of saying that the
                            // connection is not actually ready to perform this I/O operation.
                            Err(ProtocolError::Io(ref err))
                                if err.kind() == io::ErrorKind::WouldBlock =>
                            {
                                break
                            }
                            Err(ProtocolError::Io(ref err))
                                if err.kind() == io::ErrorKind::Interrupted =>
                            {
                                continue;
                            }
                            Err(ProtocolError::Io(ref err))
                                if err.kind() == io::ErrorKind::UnexpectedEof =>
                            {
                                // Disconnected!
                                client_disconnect = true;
                                break;
                            }
                            Err(ProtocolError::Bincode(ref err)) => {
                                // Sent invalid formatted packet so we'll just assume disconnected!
                                client_disconnect = true;
                                break;
                            }
                            Err(ProtocolError::Disconnected) => {
                                // Sent invalid formatted packet so we'll just assume disconnected!
                                client_disconnect = true;
                                break;
                            }
                            // Other errors we'll consider fatal for that connection.
                            Err(err) => {
                                error!("{:?}", err);
                                client_disconnect = true;
                                break;
                            }
                        }
                    }
                }

                if client_disconnect {
                    warn!(
                        "Disconnected Client {:?}: Unexpected Disconnection",
                        event.token()
                    );
                    // Remove from clients list
                    if let Some(mut client) = system.clients.remove(&UserId(token.0 as u64)) {
                        stream
                            .poll
                            .registry()
                            .deregister(&mut client.stream.stream)
                            .unwrap();
                    }
                }
            }
        }
    }
}

pub fn send_packets(mut system: ResMut<TransportSystem>, mut packets: EventReader<SendPacket>) {
    for packet in packets.iter() {
        debug!("<- {:?}", packet.0);
        if let Some(mut user) = system.clients.get_mut(&packet.1) {
            match user.stream.write_packet(packet) {
                Ok(_) => {}
                Err(e) => {
                    if e.kind() == io::ErrorKind::WouldBlock {
                        continue;
                    }
                    if e.kind() == io::ErrorKind::Interrupted {
                        error!("Connection interrupted while writing to  client");
                        continue;
                    }
                    error!("{:?}", e);
                    user.disconnected = true;
                }
                _ => {}
            }
        }
    }
}

/// Sends ping requests to check if the server is still connected
pub fn check_connections(
    mut system: ResMut<TransportSystem>,
    mut ping_requests: EventReader<ReceivePacket>,
) {
    for (uid, mut stream) in &mut system.clients {
        // If ping hasn't been sent in the last PING_TIME_SECONDS, then send it
        if Ping::new().code - stream.last_ping.code > PING_TIME_SECONDS {
            // Send new ping request
            stream
                .stream
                .write_packet(&Protocol::Ping(Ping::new()))
                .unwrap();
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
    for req in ping_requests.iter() {
        if let ReceivePacket(Protocol::Pong(req), user) = req {
            system.clients.get_mut(user).unwrap().last_pong = *req;
        }
    }
}
