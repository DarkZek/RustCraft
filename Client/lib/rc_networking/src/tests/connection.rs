use std::io::Read;
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::thread;
use std::time::Duration;
use rc_protocol::protocol::clientbound::ping::Ping;
use rc_protocol::protocol::Protocol;
use rc_protocol::types::SendPacket;
use crate::server::ServerSocket;

#[test]
fn connection_success() -> Result<(), String> {
    let ip = IpAddr::from_str("0.0.0.0").unwrap();

    let socket = ServerSocket::listen(ip, 27000).unwrap();

    let _connection = TcpStream::connect("127.0.0.1:27000").unwrap();

    socket.shutdown();

    Ok(())
}

#[test]
fn connection_event() -> Result<(), String> {
    let ip = IpAddr::from_str("0.0.0.0").unwrap();

    let mut socket = ServerSocket::listen(ip, 27001).unwrap();

    let connection = TcpStream::connect("127.0.0.1:27001").unwrap();

    thread::sleep(Duration::from_millis(15));
    let res = socket.poll();

    assert_eq!( res.connections.len(), 1);

    socket.shutdown();

    Ok(())
}

#[test]
fn send_packet() -> Result<(), String> {
    let ip = IpAddr::from_str("0.0.0.0").unwrap();

    let mut socket = ServerSocket::listen(ip, 27002).unwrap();

    let mut connection = TcpStream::connect("127.0.0.1:27002").unwrap();

    thread::sleep(Duration::from_millis(15));
    let res = socket.poll();

    let packet = Protocol::Ping(Ping::from(0));

    socket.send_packet(SendPacket(packet.clone(), res.connections.get(0).unwrap().user));

    // Send data
    socket.poll();

    thread::sleep(Duration::from_millis(15));

    let mut data = [0; 4]; // 4 Is the size of u32
    connection.read_exact(&mut data).unwrap();

    let packet_size = bincode::deserialize::<u32>(&data).unwrap();

    assert_eq!(packet_size, bincode::serialize(&packet).unwrap().len() as u32);

    socket.shutdown();

    Ok(())
}

#[test]
fn multiple_connections() -> Result<(), String> {
    let ip = IpAddr::from_str("0.0.0.0").unwrap();

    let mut socket = ServerSocket::listen(ip, 27003).unwrap();

    let connections = [
        TcpStream::connect("127.0.0.1:27003").unwrap(),
        TcpStream::connect("127.0.0.1:27003").unwrap(),
        TcpStream::connect("127.0.0.1:27003").unwrap(),
        TcpStream::connect("127.0.0.1:27003").unwrap(),
        TcpStream::connect("127.0.0.1:27003").unwrap(),
        TcpStream::connect("127.0.0.1:27003").unwrap(),
    ];

    thread::sleep(Duration::from_millis(15));
    let res = socket.poll();

    assert_eq!( res.connections.len(), 6);

    socket.shutdown();

    Ok(())
}