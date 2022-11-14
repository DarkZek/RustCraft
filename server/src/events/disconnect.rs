use crate::systems::authorization::GameUser;
use rc_client::rc_protocol::constants::UserId;

pub struct DisconnectionEvent {
    pub client: UserId,
    pub user: GameUser,
}
