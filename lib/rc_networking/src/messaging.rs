use bevy::app::App;
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};
use bevy::ecs::event::Event;
use bevy::ecs::system::SystemParam;

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


#[cfg(not(feature = "server"))]
use client as inner;

#[cfg(feature = "server")]
use server as inner;

use inner::*;

#[macro_export]
macro_rules! impl_message {
        ($typ: ty, $p_id: literal, c_id: literal) => {
            impl Message for $typ {
                const PACKET_ID: u8 = $p_id;
                const CHANNEL_ID: u8 = $c_id;
            }
        }
    }

pub trait Message: Event + Serialize + for<'a> Deserialize<'a> {
    const PACKET_ID: u8;
    const CHANNEL_ID: u8;
}

#[derive(SystemParam)]
pub struct MessageSender<'w, 's, T: Message> {
    val: EventWriter<'w, 's, Send<T>>
}

impl<T: Message> MessageSender<'_, '_, T> {
    pub fn send_default(&mut self) where T: Default {
        self.val.send_default();
    }
}

#[derive(SystemParam)]
pub struct MessageReceiver<'w, 's, T: Message> {
    val: EventReader<'w, 's, Recv<T>>
}

impl<T: Message> MessageReceiver<'_, '_, T> {
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

#[cfg(not(feature = "server"))]
mod client {
    use crate::messaging::{Message, MessageReceiver, MessageSender};

    #[derive(Default)]
    pub struct Send<T>(pub T);

    #[derive(Default)]
    pub struct Recv<T>(pub T);

    impl<T: Message> MessageSender<'_, '_, T> {
        pub fn send(&mut self, t: T) {
            self.val.send(Send(t));
        }

        pub fn send_batch(&mut self, events: impl IntoIterator<Item=T>) {
            self.val.send_batch(events.into_iter().map(|v| Send(v)));
        }
    }

    impl<T: Message> MessageReceiver<'_, '_, T> {
        pub fn iter(&mut self) -> impl DoubleEndedIterator + ExactSizeIterator + '_ {
            self.val.iter().map(|v| &v.0)
        }

        pub fn iter_with_id(&mut self) -> impl DoubleEndedIterator + ExactSizeIterator + '_ {
            self.val.iter_with_id().map(|(v, e)| (&v.0, e))
        }
    }
}

#[cfg(feature = "server")]
mod server {
    use bevy::ecs::event::EventId;
    use crate::messaging::{Message, MessageReceiver, MessageSender};

    #[derive(Default)]
    pub struct Send<T>(pub T, pub u64);

    #[derive(Default)]
    pub struct Recv<T>(pub T, pub u64);

    impl<T: Message> MessageSender<'_, '_, T> {
        pub fn send(&mut self, t: T, client: impl Into<u64>) {
            self.val.send(Send(t, client.into()))
        }

        pub fn send_batch(&mut self, events: impl IntoIterator<Item=(T, impl Into<u64>)>) {
            let iter = events
                .into_iter()
                .map(|(v, e)| {
                    Send(v, e.into())
                });
            self.val.send_batch(iter);
        }
    }

    impl<T: Message> MessageReceiver<'_, '_, T> {
        pub fn iter(&mut self) -> impl DoubleEndedIterator<Item=(&T, u64)> + ExactSizeIterator<Item=(&T, u64)> + '_ {
            self.val
                .iter()
                .map(|v| {
                    (&v.0, v.1)
                })
        }

        pub fn iter_with_id(&mut self) -> impl DoubleEndedIterator<Item=(&T, u64, EventId<Recv<T>>)> + ExactSizeIterator<Item=(&T, u64, EventId<Recv<T>>)> + '_ {
            self.val.iter_with_id()
                .map(|(v, e)| {
                    (&v.0, v.1, e)
                })
        }
    }
}