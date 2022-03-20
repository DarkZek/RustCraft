use crate::protocol::packet::serverbound::ServerBoundPacketData;

pub enum NetworkingMessage {
    // URL, Port, Username
    Connect(String, u32, String),
    Disconnect,
    PingRequest(String, u32),
    PacketQueue(Vec<ServerBoundPacketData>),
    Shutdown,
}
