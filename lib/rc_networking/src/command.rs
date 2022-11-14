use rc_protocol::constants::UserId;

pub enum NetworkCommand {
    // Used to disconnect single users
    Disconnect(UserId),
    // Used to stop the server from listening to new requests
    Stop,
}
