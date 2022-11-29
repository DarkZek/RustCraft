use std::io::Read;
use bevy::prelude::*;
use crate::messaging::{Message, PacketIdType};
use crate::client::Client;
use crate::server::Server;
use crate::messaging::server;
use crate::messaging::client;

const fn size_of_packet_id() -> usize {
    std::mem::size_of::<PacketIdType>()
}

fn message_size<T: Message>() -> usize {
    use std::mem::size_of;
    size_of::<T>() + size_of_packet_id()
}

pub fn serialize<T: Message>(value: &T) -> Vec<u8> {
    use std::io::{Write, Cursor};

    let packet_size = message_size::<T>();
    let mut bytes = Cursor::new(Vec::with_capacity(packet_size));

    bytes.write(&T::PACKET_ID.to_le_bytes()).unwrap();
    bincode::serialize_into(&mut bytes, &value).unwrap();

    bytes.into_inner()
}

pub fn deserialize_packet_id(read: &mut impl Read) -> PacketIdType {
    let mut packet_id = [0u8; size_of_packet_id()];
    read.read(&mut packet_id).unwrap();
    PacketIdType::from_le_bytes(packet_id)
}

pub fn deserialize<T: Message>(read: impl Read) -> T {
    bincode::deserialize_from(read).unwrap()
}

pub fn message_write_server<T: Message>(world: &mut World, server: &mut Server) {
    let events = world.resource_mut::<Events<server::SendMsg<T>>>();
    let mut reader = events.get_reader();
    for msg in reader.iter(&events) {
        let bytes = serialize(&msg.0);
        server.send_message(msg.1, T::CHANNEL_ID, bytes);
    }
}

pub fn message_write_client<T: Message>(world: &mut World, server: &mut Client) {
    let events = world.resource_mut::<Events<client::SendMsg<T>>>();
    let mut reader = events.get_reader();
    for msg in reader.iter(&events) {
        let bytes = serialize(&msg.0);
        server.send_message( T::CHANNEL_ID, bytes);
    }
}

macro_rules! make_serializer {
        ($($typ:ty),*) => {
            use std::io::{Cursor, Read};
            use crate::messaging::Message;
            use crate::messaging::serialize::*;
            #[allow(unused)]
            pub fn client_de(world: &mut World, bytes: Vec<u8>) {
                make_deserializers!(@body bytes, world, {,}, $($typ),*);
            }
            #[allow(unused)]
            pub fn server_de(world: &mut World, bytes: Vec<u8>, client_id: u64) {
                make_deserializers!(@body bytes, world, {client_id}, $($typ),*);
            }
            #[allow(unused)]
            pub fn client_ser(world: &mut World, client: &mut Client) {
                $(message_write_client::<$typ>(world, client);)*
            }
            #[allow(unused)]
            pub fn server_ser(world: &mut World, server: &mut Server) {
                $(message_write_server::<$typ>(world, server);)*
            }
        };
        (@body {$($_c_id:tt)*} $(,)?) => {

        };
        (@body $bytes:ident, $world:ident, {$($c_id:tt)*}, $($typ:ty),*) => {
            let mut read = Cursor::new($bytes);
            let id = deserialize_packet_id(&mut read);

            match id {
                $(<$typ>::PACKET_ID => {
                    let val = deserialize::<$typ>(read);
                    let event = crate::messaging::make_recv!(val, $c_id);
                    $world.send_event(event);
                })*
                _ => { unreachable!("Packet with Id {} is unknown", id); }
            }
        };
    }
pub(crate) use make_serializer;
