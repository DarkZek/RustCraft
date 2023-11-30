use crate::systems::connection::authorization::authorization_event;
use crate::systems::connection::disconnect::disconnection_event;
use crate::systems::connection::finish_join::detect_finish_join;
use crate::systems::connection::message::receive_message_event;
use crate::{App, Update};
use bevy::prelude::{Plugin};
use rc_networking::constants::{GameObjectId, UserId};

mod authorization;
mod disconnect;
mod finish_join;
mod message;

pub struct ConnectionPlugin;

impl Plugin for ConnectionPlugin {
    fn build(&self, app: &mut App) {
        // Receive Server Events
        app.add_systems(Update, authorization_event)
            .add_systems(Update, disconnection_event)
            .add_systems(Update, receive_message_event)
            .add_systems(Update, detect_finish_join);
    }
}

/// A user who is yet to be authorized
pub struct GameUser {
    pub name: Option<String>,

    pub user_id: UserId,
    pub game_object_id: GameObjectId,
    pub loading: bool,
}

impl GameUser {
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }
}
