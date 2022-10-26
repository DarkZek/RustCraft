use std::io;
use bevy::log::{info, warn};
use rustcraft_protocol::constants::UserId;
use rustcraft_protocol::error::ProtocolError;
use rustcraft_protocol::protocol::{Protocol, ReceivePacket};
use rustcraft_protocol::protocol::serverbound::pong::Pong;
use crate::{EventReader, EventWriter, ResMut};
use crate::services::networking::transport::listener::ClientListener;
use crate::services::networking::TransportSystem;

pub fn connection_upkeep(system: ResMut<TransportSystem>, mut stream: ResMut<ClientListener>, mut event_writer: EventWriter<ReceivePacket>) {

    // Check if we're connected to a server yet
    if stream.stream.is_none() {
        return;
    }

    let mut client_disconnect = false;

    loop {
        let mut data = vec![0u8; 4];
        match stream.stream.as_mut().unwrap().stream.peek(&mut data) {
            Ok(val) => { info!("{:?} {:?}", val, data);},
            Err(e) => { break; }
        }

        match stream.stream.as_mut().unwrap().read_packet() {
            Ok(n) => {

                info!("{:?}", n);

                // Respond to pings
                if let Protocol::Ping(ping) = &n {
                    stream.stream.as_mut().unwrap().write_packet(&Protocol::Pong(Pong::from(ping.code))).unwrap();
                } else {
                    event_writer.send(ReceivePacket(n, UserId(0)));
                }
            }
            // Would block "errors" are the OS's way of saying that the
            // connection is not actually ready to perform this I/O operation.
            Err(ProtocolError::Io(ref err)) if err.kind() == io::ErrorKind::WouldBlock => break,
            Err(ProtocolError::Io(ref err)) if err.kind() == io::ErrorKind::Interrupted => continue,
            Err(ProtocolError::Io(ref err)) if err.kind() == io::ErrorKind::UnexpectedEof => {
                // I'm not sure if this should be an error?
                warn!("{:?}", err);
                // Disconnected!
                client_disconnect = true;
                break;
            },
            Err(ProtocolError::Bincode(ref err)) => {
                warn!("Bincode {:?}", err);
                // Sent invalid formatted packet so we'll just assume disconnected!
                client_disconnect = true;
                break;
            },
            Err(ProtocolError::Disconnected) => {
                // Sent invalid formatted packet so we'll just assume disconnected!
                client_disconnect = true;
                break;
            },
            // Other errors we'll consider fatal.
            Err(err) => panic!("{:?}", err),
        }
    }

    if client_disconnect {
        warn!("Disconnected from server: Unexpected Disconnection");
        stream.stream = None;
    }
}