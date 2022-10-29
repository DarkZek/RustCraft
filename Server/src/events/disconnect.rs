use crate::systems::authorization::GameUser;
use rustcraft_protocol::constants::UserId;

pub struct DisconnectionEvent {
    pub client: UserId,
    pub user: GameUser,
}
