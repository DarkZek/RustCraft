use crate::messaging::NetworkingMessage;
use crate::protocol::decoder::PacketDecoder;
use crate::protocol::login::LoginRequest;
use crate::protocol::packet::Packet;
use crate::protocol::status::PingRequest;
use crate::stream::NetworkStream;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use std::{mem, process, thread};

pub mod messaging;
pub mod protocol;
pub mod stream;

#[derive(Clone)]
pub struct RustcraftNetworking {
    messaging_channel: Arc<Mutex<Vec<NetworkingMessage>>>,
    received_packets: Arc<Mutex<Vec<Packet>>>,
    send_packets: Arc<Mutex<Vec<Packet>>>,
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
        let mut messaging_channel = self.messaging_channel.clone();
        let mut received_packets = self.received_packets.clone();
        let mut send_packets = self.send_packets.clone();

        thread::spawn(move || {
            let mut context = NetworkingContext::new();
            let mut connection: Option<NetworkStream> = None;

            loop {
                // Listen to messages
                match messaging_channel.lock() {
                    Ok(mut messages) => {
                        handle_messages(&mut *messages, &mut connection);
                    }
                    Err(e) => println!("{}", e),
                }

                if connection.is_none() {
                    continue;
                }

                let packet = PacketDecoder::decode(&mut connection.as_mut().unwrap());

                match received_packets.lock() {
                    Ok(mut received_packets) => {
                        received_packets.push(packet);
                    }
                    Err(e) => println!("{}", e),
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

    pub fn get_packets(&mut self) -> Vec<Packet> {
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
