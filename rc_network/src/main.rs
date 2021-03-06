#![feature(once_cell)]

use rc_network::messaging::NetworkingMessage;
use rc_network::RustcraftNetworking;
use std::io;
use std::io::BufRead;

// Based on protocol as of
// https://web.archive.org/web/20200601221423/https://wiki.vg/Protocol

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
