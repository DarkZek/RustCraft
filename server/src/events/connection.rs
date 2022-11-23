use rc_networking::constants::UserId;

pub struct ConnectionEvent {
    pub user: UserId,
}

impl ConnectionEvent {
    pub fn new(user: UserId) -> ConnectionEvent {
        ConnectionEvent {
            user
        }
    }
}