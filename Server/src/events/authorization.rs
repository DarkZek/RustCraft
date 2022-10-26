use rustcraft_protocol::constants::UserId;

pub struct AuthorizationEvent {
    pub client: UserId,
}