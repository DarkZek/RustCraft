use rc_network::messaging::NetworkingMessage;
use rc_network::RustcraftNetworking;
use std::io;
use std::io::BufRead;

// Simple rc_network setup for testing
fn main() {
    let networking = RustcraftNetworking::new();

    networking.start();

    networking.send_message(NetworkingMessage::Connect("localhost".to_string(), 25565));

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();

        if line == "connect".to_string() {
            networking.send_message(NetworkingMessage::Connect("localhost".to_string(), 25565));
        }

        if line == "exit".to_string() {
            break;
        }
    }
}
