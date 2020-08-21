pub enum NetworkingMessage {
    Connect(String, u32),
    Disconnect,
    PingRequest(String, u32),
    Shutdown,
}
