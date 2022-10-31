use crate::services::networking::transport::listener::ClientListener;
use crate::services::networking::transport::packet::{ReceivePacket, SendPacket};

use crate::{debug, EventReader, EventWriter, ResMut};
use bevy::log::{info, warn};
use bevy::prelude::error;

use rustcraft_protocol::error::ProtocolError;
use rustcraft_protocol::protocol::serverbound::pong::Pong;
use rustcraft_protocol::protocol::Protocol;
use std::io;

pub fn connection_upkeep(
    mut stream: ResMut<ClientListener>,
    mut event_writer: EventWriter<ReceivePacket>,
) {
    // Check if we're connected to a server yet
    if stream.stream.is_none() {
        return;
    }

    let mut client_disconnect = stream.disconnect;

    loop {
        let mut data = vec![0u8; 4];
        match stream.stream.as_mut().unwrap().stream.peek(&mut data) {
            Ok(v) => {
                if v == 0 {
                    warn!("No readings");
                }
            }
            Err(_) => {
                break;
            }
        }

        match stream.stream.as_mut().unwrap().read_packet() {
            Ok(n) => {
                debug!("-> {:?}", n);

                // Respond to pings
                if let Protocol::Ping(ping) = &n {
                    stream
                        .stream
                        .as_mut()
                        .unwrap()
                        .write_packet(&Protocol::Pong(Pong::from(ping.code)))
                        .unwrap();
                } else {
                    event_writer.send(ReceivePacket(n));
                }
            }
            // Would block "errors" are the OS's way of saying that the
            // connection is not actually ready to perform this I/O operation.
            Err(ProtocolError::Io(ref err)) if err.kind() == io::ErrorKind::WouldBlock => {
                info!("WouldBlock");
                break;
            }
            Err(ProtocolError::Io(ref err)) if err.kind() == io::ErrorKind::Interrupted => {
                info!("Interrupted");
                continue;
            }
            Err(ProtocolError::Io(ref err)) if err.kind() == io::ErrorKind::UnexpectedEof => {
                warn!("{:?}", err);
                // Disconnected!
                client_disconnect = true;
                break;
            }
            Err(ProtocolError::Bincode(ref err)) => {
                warn!("Bincode {:?}", err);
                // Sent invalid formatted packet so we'll just assume disconnected!
                client_disconnect = true;
                break;
            }
            Err(ProtocolError::Disconnected) => {
                // Sent invalid formatted packet so we'll just assume disconnected!
                client_disconnect = true;
                break;
            }
            // Other errors we'll consider fatal.
            Err(err) => panic!("{:?}", err),
        }
    }

    if client_disconnect {
        warn!("Disconnected from server: Unexpected Disconnection");
        stream.stream = None;
    }
}

pub fn send_packets(mut stream: ResMut<ClientListener>, mut packets: EventReader<SendPacket>) {
    if stream.stream().is_none() {
        return;
    }

    for packet in packets.iter() {
        debug!("<- {:?}", packet.0);
        match stream.stream.as_mut().unwrap().write_packet(packet) {
            Ok(_) => {}
            Err(e) => {
                if e.kind() == io::ErrorKind::NotConnected {
                    // Tried to send packets before connection.
                    warn!("Tried to send packets before connection.");
                    return;
                }
                error!("Error during packet send {:?}", e);
                stream.disconnect = true;
            }
        }
    }
}
