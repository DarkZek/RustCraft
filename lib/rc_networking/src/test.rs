use std::io::Cursor;
use bevy::prelude::{Events, EventWriter, ResMut, World};
use crate::*;
use crate::messaging::Message;

pub struct PacketTypes {

}

use crate::messaging::server::SendMsg;
use crate::messaging::server::RecvMsg;

// requires exclusive world access, defer calls until end of network start stage
fn read_message<T: Message>(client_id: u64, bytes: &[u8], world: &mut World) {
    let v: T = bincode::deserialize::<T>(bytes).unwrap();
    let event = RecvMsg(v, client_id);
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

pub struct RawMessage {
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

pub fn message_update_system<T: Message>(mut events: ResMut<Events<SendMsg<T>>>, mut raw: EventWriter<RawMessage>) {
    let mut reader = events.get_reader();
    reader.iter(&events)
        .for_each(|msg: &SendMsg<T>| {
            use std::mem::{size_of, size_of_val};
            let size = size_of::<T>() + size_of_val(&T::PACKET_ID);
            let mut bytes = Cursor::new(Vec::with_capacity(size));
            bytes.get_mut().push(T::PACKET_ID);
            bincode::serialize_into(&mut bytes, &msg.0).unwrap(); // HERE BE SERIALIZATION
            raw.send(RawMessage::new(msg.1, T::CHANNEL_ID, bytes.into_inner()));
        });
    events.update();
}

pub fn write_packets_system(
    mut server: ResMut<Server>,
    mut raw: ResMut<Events<RawMessage>>,
) {
    for msg in raw.drain() {
        server.send_message(msg.client, msg.channel, msg.bytes);
    }
}