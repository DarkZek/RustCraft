use std::io::{Cursor, Read};
use bevy::app::App;
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};
use bevy::ecs::event::Event;
use bevy::ecs::system::SystemParam;
use deserialize::make_deserializers;

make_deserializers![

];

// special case: the player's entity should be the given id from renet
#[derive(Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Debug, Default, Component, Serialize, Deserialize)]
pub struct NetworkEntity(pub u64);

impl From<NetworkEntity> for u64 {
    fn from(value: NetworkEntity) -> Self {
        value.0
    }
}

#[derive(Resource, Default)]
pub struct NetworkEntities {
    map: HashMap<NetworkEntity, Entity>,
}

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NetworkEntities>();
    }
}

#[macro_export]
macro_rules! impl_message {
    ($typ: ty, $p_id: literal, $c_id: literal) => {
        impl Message for $typ {
            const PACKET_ID: PacketIdType = $p_id;
            const CHANNEL_ID: u8 = $c_id;
        }
    }
}

pub type PacketIdType = u8;

pub trait Message: Event + Serialize + for<'a> Deserialize<'a> {
    const PACKET_ID: PacketIdType;
    const CHANNEL_ID: u8;
}

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
    use std::io::{Read, Cursor};
    bincode::deserialize_from(read).unwrap()
}

macro_rules! make_recv {
    ($val: expr $(,)?) => { crate::messaging::client::RecvMsg($val) };
    ($val: expr, $id: expr) => { crate::messaging::server::RecvMsg($val, $id) };
}
pub(crate) use make_recv;

#[derive(SystemParam)]
pub struct Sender<'w, 's, T: Event> {
    val: EventWriter<'w, 's, T>,
}

impl<T: Event> Sender<'_, '_, T> {
    pub fn send(&mut self, t: impl Into<T>) {
        self.val.send(t.into());
    }

    pub fn send_batch(&mut self, events: impl IntoIterator<Item=impl Into<T>>) {
        let iter = events
            .into_iter()
            .map(|v| v.into());
        self.val.send_batch(iter);
    }

    pub fn send_default(&mut self) where T: Default {
        self.send_default();
    }
}

#[derive(SystemParam)]
pub struct Receiver<'w, 's, T: Event> {
    val: EventReader<'w, 's, T>,
}

impl<T: Event> Receiver<'_, '_, T> {
    pub fn len(&self) -> usize {
        self.val.len()
    }

    pub fn is_empty(&self) -> bool {
        self.val.is_empty()
    }

    pub fn clear(self) {
        self.val.clear()
    }
}

macro_rules! impl_deref {
    ($typ: ty, $inner: ty) => {
        impl<'w, 's, T: crate::messaging::Message> Deref for $typ {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl<'w, 's, T: crate::messaging::Message> DerefMut for $typ {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }
    }
}

pub mod client {
    use std::ops::{Deref, DerefMut};
    use bevy::ecs::event::EventId;
    use bevy::ecs::system::SystemParam;
    use bevy::prelude::World;
    use crate::messaging::Message;
    use crate::messaging::{Receiver, Sender};

    use crate::messaging::client_de;

    pub fn deserialize(world: &mut World, bytes: Vec<u8>) {
        client_de(bytes, world);
    }

    pub struct SendMsg<T>(pub T);

    impl<T> From<T> for SendMsg<T> {
        fn from(value: T) -> Self {
            Self(value)
        }
    }

    pub struct RecvMsg<T>(pub T);

    #[derive(SystemParam)]
    pub struct MessageSender<'w, 's, T: Message> {
        inner: Sender<'w, 's, SendMsg<T>>,
    }

    impl_deref!(MessageSender<'w, 's, T>, Sender<'w, 's, SendMsg<T>>);

    #[derive(SystemParam)]
    pub struct MessageReceiver<'w, 's, T: Message> {
        inner: Receiver<'w, 's, RecvMsg<T>>,
    }

    impl_deref!(MessageReceiver<'w, 's, T>, Receiver<'w, 's, RecvMsg<T>>);

    impl<T: Message> MessageReceiver<'_, '_, T> {
        pub fn iter(&mut self) -> impl DoubleEndedIterator<Item=&T> + ExactSizeIterator + '_ {
            self.val
                .iter()
                .map(|v| &v.0)
        }

        pub fn iter_with_id(&mut self) -> impl DoubleEndedIterator<Item=(&T, EventId<RecvMsg<T>>)> + ExactSizeIterator + '_ {
            self.val
                .iter_with_id()
                .map(|(v, e)| (&v.0, e))
        }
    }
}

pub mod server {
    use std::ops::{Deref, DerefMut};
    use bevy::ecs::event::EventId;
    use bevy::ecs::system::SystemParam;
    use bevy::prelude::World;
    use crate::messaging::Message;
    use crate::messaging::{Receiver, Sender};

    use crate::messaging::server_de;

    pub fn deserialize(world: &mut World, bytes: Vec<u8>, client_id: u64) {
        server_de(bytes, world, client_id);
    }

    pub struct SendMsg<T>(pub T, pub u64);

    impl<T> From<(T, u64)> for SendMsg<T> {
        fn from(value: (T, u64)) -> Self {
            Self(value.0, value.1)
        }
    }

    pub struct RecvMsg<T>(pub T, pub u64);

    #[derive(SystemParam)]
    pub struct MessageSender<'w, 's, T: Message> {
        inner: Sender<'w, 's, SendMsg<T>>,
    }

    impl_deref!(MessageSender<'w, 's, T>, Sender<'w, 's, SendMsg<T>>);

    #[derive(SystemParam)]
    pub struct MessageReceiver<'w, 's, T: Message> {
        inner: Receiver<'w, 's, RecvMsg<T>>,
    }

    impl_deref!(MessageReceiver<'w, 's, T>, Receiver<'w, 's, RecvMsg<T>>);

    impl<T: Message> MessageReceiver<'_, '_, T> {
        pub fn iter(&mut self) -> impl DoubleEndedIterator<Item=(&T, u64)> + ExactSizeIterator + '_ {
            self.val
                .iter()
                .map(|v| (&v.0, v.1))
        }

        pub fn iter_with_id(&mut self) -> impl DoubleEndedIterator<Item=(&T, u64, EventId<RecvMsg<T>>)> + ExactSizeIterator + '_ {
            self.val
                .iter_with_id()
                .map(|(v, e)| (&v.0, v.1, e))
        }
    }
}

mod deserialize {
    macro_rules! make_deserializers {
        ($($typ:ty),*) => {
            make_deserializers!(client_de, {}, {}, $($typ),*);
            make_deserializers!(server_de, {client_id}, {client_id: u64}, $($typ),*);
        };
        ($name:ident, {$($c_id:tt)*}, {$($param:tt)*}, $($typ:ty),+) => {
            fn $name(bytes: Vec<u8>, world: &mut World, $($param)*) {
                let mut read = Cursor::new(bytes);
                let id = deserialize_packet_id(&mut read);

                match id {
                    $(<$typ>::PACKET_ID => {
                        let val = deserialize::<$typ>(read);
                        let event = make_recv!(val, $c_id);
                        world.send_event(event);
                    })*
                    _ => { unreachable!("Packet with Id {} is unknown", id); }
                }
            }
        };
        ($name:ident, {$($_c_id:tt)*}, {$($param:tt)*} $(,)?) => {
            fn $name(bytes: Vec<u8>, world: &mut World, $($param)*) {

            }
        }
    }
    pub(crate) use make_deserializers;
}
