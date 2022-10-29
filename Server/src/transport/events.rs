
use tokio::net::TcpStream;

pub enum NetworkEvent {
    ConnectionSuccessful(TcpStream),
    ConnectionFailed,
}
