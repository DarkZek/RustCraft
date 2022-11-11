use rustcraft_protocol::constants::UserId;
use rustcraft_protocol::protocol::Protocol;

/// Alias used to differentiate the packets for use with Bevy's ECS Event Readers
#[derive(Clone)]
pub struct SendPacket(pub Protocol, pub UserId);

impl std::ops::Deref for SendPacket {
    type Target = Protocol;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Alias used to differentiate the packets for use with Bevy's ECS Event Readers
#[derive(Clone)]
pub struct ReceivePacket(pub Protocol, pub UserId);

impl std::ops::Deref for ReceivePacket {
    type Target = Protocol;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
