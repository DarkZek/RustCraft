use bevy::ecs::event::Event;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

pub mod client;
pub mod server;

mod serialize;
mod messages;

#[macro_export]
macro_rules! impl_message {
    ($typ: ty, $p_id: expr, $c_id: expr) => {
        impl crate::messaging::Message for $typ {
            const PACKET_ID: crate::messaging::PacketIdType = $p_id;
            const CHANNEL_ID: u8 = $c_id;
        }
    }
}

pub type PacketIdType = u8;

pub trait Message: Event + Serialize + for<'a> Deserialize<'a> {
    const PACKET_ID: PacketIdType;
    const CHANNEL_ID: u8;
}

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

#[derive(SystemParam)]
pub struct Sender<'w, 's, T: Event> {
    val: EventWriter<'w, 's, T>,
}

#[allow(unused)]
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
        self.val.send_default();
    }
}

#[derive(SystemParam)]
pub struct Receiver<'w, 's, T: Event> {
    val: EventReader<'w, 's, T>,
}

#[allow(unused)]
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

#[allow(unused)]
macro_rules! make_recv {
    ($val: expr $(,)*) => { crate::messaging::client::RecvMsg($val) };
    ($val: expr, $id: expr) => { crate::messaging::server::RecvMsg($val, $id) };
}
#[allow(unused)]
pub(crate) use make_recv;

macro_rules! impl_deref {
    ($typ: ty, $inner: ty) => {
        impl<'w, 's, T: crate::messaging::Message> std::ops::Deref for $typ {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl<'w, 's, T: crate::messaging::Message> std::ops::DerefMut for $typ {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }
    }
}
pub(crate) use impl_deref;