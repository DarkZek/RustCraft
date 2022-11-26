use bevy::ecs::event::EventId;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use crate::client::Client;
use crate::messaging::{impl_deref, Message, Receiver, Sender};
use crate::messaging::messages::{client_de, client_ser};

pub fn deserialize(world: &mut World, bytes: Vec<u8>) {
    client_de(world, bytes);
}

pub fn serialize(world: &mut World, client: &mut Client) {
    client_ser(world, client);
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