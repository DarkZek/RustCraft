use crate::systems::authorization::GameUser;
use rc_client::rc_networking::constants::UserId;

pub struct DisconnectionEvent {
    pub client: UserId,
    pub user: GameUser,
}
