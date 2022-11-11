use rc_client::rc_protocol::constants::UserId;

pub struct AuthorizationEvent {
    pub client: UserId,
}

impl AuthorizationEvent {
    pub fn new(client: UserId) -> AuthorizationEvent {
        AuthorizationEvent { client }
    }
}
