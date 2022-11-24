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
    use crate::messaging::Message;
    use crate::messaging::{Receiver, Sender};

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
    use crate::messaging::Message;
    use crate::messaging::{Receiver, Sender};

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
