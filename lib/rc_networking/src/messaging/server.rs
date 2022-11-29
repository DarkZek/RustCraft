use bevy::ecs::event::EventId;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use crate::messaging::{impl_deref, Message, Receiver, Sender};
use crate::messaging::messages::{server_de, server_ser};
use crate::server::Server;

pub fn deserialize(world: &mut World, bytes: Vec<u8>, client_id: u64) {
    server_de(world, bytes, client_id);
}

pub fn serialize(world: &mut World, server: &mut Server) {
    server_ser(world, server);
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

#[allow(unused)]
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