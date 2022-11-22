use bevy::app::App;
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};
use crate::message::{MessageReceiver, MessageSender};

pub mod constants;
pub mod protocol;
pub mod types;

// special case: the player's entity should be the given id from renet
#[derive(Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Debug, Default, Component, Serialize, Deserialize)]
pub struct NetworkEntity(pub u64);

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

#[derive(Component, Default)]
pub struct SyncPosition {
    val: Vec3,
}

pub struct SyncPositionCommand {
    id: NetworkEntity,
    pos: Vec3,
}

pub fn send_position_system(
    mut query: Query<(&NetworkEntity, &Transform, &mut SyncPosition)>,
    mut sender: MessageSender<SyncPositionCommand>,
) {
    for (net, transform, mut sync) in query.iter_mut() {
        if transform.translation.abs_diff_eq(sync.val, 0.01) {
            sync.val = transform.translation;
            sender.send(SyncPositionCommand{ id: *net, pos: sync.val });
        }
    }
}

pub fn recv_position_system (
    mut query: Query<(With<NetworkEntity>, With<Transform>, &mut SyncPosition)>,
    mut receiver: MessageReceiver<SyncPositionCommand>,
    net_entities: Res<NetworkEntities>,
) {
    for msg in receiver.iter() {
        if let Some(entity) = net_entities.map.get(&msg.id) {
            let (_, _, mut sync) = query.get_mut(*entity).unwrap();
            sync.val = msg.pos;
        }
    }
}

mod message {
    use bevy::app::App;
    use bevy::ecs::event::{EventWriter, EventReader, Events, Event, EventId};
    use bevy::ecs::system::SystemParam;

    pub fn add_message<T: Event>(app: &mut App) {
        if !app.world.contains_resource::<Events<T>>() {
            app.init_resource::<Events<T>>();
        }
    }

    #[derive(Default)]
    pub struct Sender<T: Event>(T);

    #[derive(SystemParam)]
    pub struct MessageSender<'w, 's, T: Event> {
        val: EventWriter<'w, 's, Sender<T>>
    }

    impl<T: Event> MessageSender<'_, '_, T> {
        pub fn send(&mut self, t: T) {
            self.val.send(Sender(t));
        }

        pub fn send_batch(&mut self, events: impl IntoIterator<Item=T>) {
            self.val.send_batch(events.into_iter().map(|v| Sender(v)));
        }

        pub fn send_default(&mut self) where T: Default {
            self.val.send_default()
        }
    }

    pub struct Receiver<T: Event>(T);

    #[derive(SystemParam)]
    pub struct MessageReceiver<'w, 's, T: Event> {
        val: EventReader<'w, 's, Receiver<T>>
    }

    impl<T: Event> MessageReceiver<'_, '_, T> {
        pub fn iter(&mut self) -> impl DoubleEndedIterator<Item=&T> + ExactSizeIterator<Item=&T> + '_ {
            self.val.iter()
                .map(|v| &v.0)
        }

        pub fn iter_with_id(&mut self) -> impl DoubleEndedIterator<Item=(&T, EventId<Receiver<T>>)> + ExactSizeIterator<Item=(&T, EventId<Receiver<T>>)> + '_ {
            self.val.iter_with_id()
                .map(|(v, e)| (&v.0, e))
        }

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
}
