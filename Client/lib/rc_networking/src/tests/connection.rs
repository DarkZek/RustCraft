use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use crate::server::ServerSocket;

#[test]
fn connection_success() -> Result<(), String> {
    let ip = IpAddr::from_str("127.0.0.1").unwrap();

    let _socket = ServerSocket::listen(ip, 27000).unwrap();

    let _connection = TcpStream::connect("127.0.0.1:27000").unwrap();

    Ok(())
}