use bevy::prelude::{Events, ResMut, World};
use rc_protocol::message::{Message, Receiver};
use crate::*;

// requires exclusive world access, defer calls until end of network start stage
fn read_message<T: Message>(bytes: &[u8], world: &mut World) {
    let v: T = bincode::deserialize::<T>(bytes).unwrap();
    let event = Receiver(v);
    world.send_event(event);
}

fn read_packets_system(
    mut server: ResMut<Server>,
) {
    server.clients_id()
        .iter()
        .for_each(|&user| {
            while let Some(bytes) = server.receive_message(user, Channel::Reliable) {

            }
        })
}

struct RawMessage {
    client: u64,
    channel: u8,
    bytes: Vec<u8>,
}

impl RawMessage {
    fn new(client: impl Into<u64>, channel: impl Into<u8>, bytes: Vec<u8>) -> Self {
        Self {
            client: client.into(),
            channel: channel.into(),
            bytes,
        }
    }
}

fn write_message<T: Message>(msg: T, raw: &mut Events<RawMessage>) {
    let bytes = bincode::serialize(&msg).unwrap();
    //raw.send(RawMessage::new(client?????, channel?????, bytes));
}

fn write_packets_system(
    mut server: ResMut<Server>,
    mut raw: ResMut<Events<RawMessage>>,
) {
    for msg in raw.drain() {
        server.send_message(msg.client, msg.channel, msg.bytes);
    }
}