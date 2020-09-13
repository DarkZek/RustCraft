use crate::messaging::NetworkingMessage;
use crate::protocol::decoder::PacketDecoder;
use crate::stream::NetworkStream;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use std::{mem, process, thread};

use crate::protocol::data::writer::PacketBuilder;
use crate::protocol::packet::PacketData;
use crate::protocol::requests::login::LoginRequest;
use crate::protocol::requests::status::PingRequest;
use byteorder::{BigEndian, WriteBytesExt};

pub mod messaging;
pub mod protocol;
pub mod stream;

#[derive(Clone)]
pub struct RustcraftNetworking {
    messaging_channel: Arc<Mutex<Vec<NetworkingMessage>>>,
    received_packets: Arc<Mutex<Vec<PacketData>>>,
    send_packets: Arc<Mutex<Vec<PacketData>>>,
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(1, 3);
    }
}

pub struct NetworkingContext {
    play_state: u8,
}

impl NetworkingContext {
    pub fn new() -> NetworkingContext {
        NetworkingContext { play_state: 0 }
    }
}

impl RustcraftNetworking {
    pub fn new() -> RustcraftNetworking {
        RustcraftNetworking {
            messaging_channel: Arc::new(Mutex::new(vec![])),
            received_packets: Arc::new(Mutex::new(vec![])),
            send_packets: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn start(&self) {
        let messaging_channel = self.messaging_channel.clone();
        let received_packets = self.received_packets.clone();
        let _send_packets = self.send_packets.clone();

        thread::spawn(move || {
            let _context = NetworkingContext::new();
            let mut connection: Option<NetworkStream> = None;

            loop {
                // Listen to messages
                match messaging_channel.lock() {
                    Ok(mut messages) => {
                        handle_messages(&mut *messages, &mut connection);
                    }
                    Err(e) => println!("Poison Error: {}", e),
                }

                if connection.is_none() {
                    continue;
                }

                match PacketDecoder::decode(&mut connection.as_mut().unwrap()) {
                    Ok(packet) => {
                        // Answer callbacks
                        if let PacketData::KeepAlive(packet) = packet {
                            // Send packet back
                            let mut builder = PacketBuilder::new(0x0F);
                            builder.data.write_i64::<BigEndian>(packet.keep_alive_id);
                            builder.send(connection.as_mut().unwrap());
                            println!("Answered callback");
                            continue;
                        }

                        // Handle disconnects
                        if let PacketData::Disconnect(packet) = packet {
                            // Send packet back
                            connection = None;
                            println!("Disconnected: {}", packet.reason);
                            continue;
                        }

                        match received_packets.lock() {
                            Ok(mut received_packets) => {
                                received_packets.push(packet);
                            }
                            Err(e) => println!("Mutex Poison Error {}", e),
                        }
                    }
                    Err(e) => {
                        println!("Error parsing packet {}", e);
                    }
                }
            }
        });
    }

    pub fn send_message(&self, message: NetworkingMessage) {
        match self.messaging_channel.lock() {
            Ok(mut messaging_channel) => messaging_channel.push(message),
            Err(_) => {}
        }
    }

    pub fn get_packets(&mut self) -> Vec<PacketData> {
        match self.received_packets.lock() {
            Ok(mut received_packets) => mem::take(received_packets.deref_mut()),
            Err(_) => Vec::new(),
        }
    }
}

fn handle_messages(messages: &mut Vec<NetworkingMessage>, connection: &mut Option<NetworkStream>) {
    for message in messages.iter_mut() {
        match message {
            NetworkingMessage::Connect(connection_host, connection_port) => {
                *connection =
                    NetworkStream::connect(format!("{}:{}", connection_host, connection_port)).ok();

                if connection.is_some() {
                    let login = LoginRequest {
                        connection_host: connection_host.clone(),
                        connection_port: *connection_port,
                    };

                    login.send(&mut connection.as_mut().unwrap());
                }
            }
            NetworkingMessage::Disconnect => {}
            NetworkingMessage::PingRequest(connection_host, connection_port) => {
                *connection =
                    NetworkStream::connect(format!("{}:{}", connection_host, connection_port)).ok();

                if connection.is_some() {
                    PingRequest::send(connection_host.clone(), connection_port.clone());
                }
            }
            NetworkingMessage::Shutdown => process::exit(1),
        }
    }

    messages.clear();
}
