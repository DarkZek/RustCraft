use std::time::Duration;
use bevy_log::{debug, error, info, warn};
use crossbeam::channel::{Receiver, Sender, unbounded};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use rc_protocol::constants::{UserId};

use rc_protocol::protocol::Protocol;

use rc_protocol::types::{ReceivePacket, SendPacket};
use crate::server::ServerSocket;
use crate::server::user::NetworkUser;

#[derive(Debug)]
pub struct ConnectionEvent {
    pub user: UserId
}

struct ConnectionRequest(pub TcpStream);

impl ServerSocket {
    pub fn new_connections(&mut self) -> Vec<ConnectionEvent> {

        let mut connections = Vec::new();

        // Loop over all new connections
        while let Ok(conn) = self.receive_connections.recv_timeout(Duration::ZERO) {

            self.lifetime_connections += 1;

            // Generate new userid
            let uid = UserId(self.lifetime_connections as u64);

            info!(
                "Connection request made from {} given UID {:?}",
                conn.0.peer_addr().unwrap(),
                uid
            );

            conn.0.set_nodelay(true).unwrap();

            let (mut read_tcp, mut write_tcp) = conn.0.into_split();

            let (inner_write_packets, read_packets) = unbounded();

            // Read packets
            let read_packet_handle = self.runtime.spawn(async move {
                loop {
                    let mut data = [0; 4]; // 4 Is the size of u32
                    match read_tcp.read_exact(&mut data).await {
                        Ok(0) => {
                            warn!("Potentially closed")
                        }
                        Ok(_) => {}
                        Err(e) => {
                            if e.kind() == std::io::ErrorKind::UnexpectedEof {
                                break;
                            }
                            error!("Error reading data from client {:?}", uid);
                            break;
                        }
                    };

                    let len: u32 = match bincode::deserialize(&data[..]) {
                        Ok(val) => val,
                        Err(e) => {
                            error!("Error reading data from client {:?}: {:?}", uid, e);
                            break;
                        }
                    };

                    let mut data = vec![0u8; len as usize];

                    // Read packet data
                    match read_tcp.read_exact(&mut data).await {
                        Ok(val) => val,
                        Err(e) => {
                            if e.kind() == std::io::ErrorKind::UnexpectedEof {
                                break;
                            }
                            error!("Error reading data from client {:?}: {:?}", uid, e);
                            break;
                        }
                    };

                    let packet: Protocol = match bincode::deserialize(&data[..]) {
                        Ok(val) => val,
                        Err(e) => {
                            error!("Error reading data from client {:?}: {:?}", uid, e);
                            break;
                        }
                    };

                    // Send packet to receiver
                    match inner_write_packets.send(ReceivePacket(packet, uid)) {
                        Ok(_) => {}
                        Err(e) => {
                            // Channel disconnected, delete this task
                            debug!(
                            "Failed to read packet for user {:?} destroyed: {:?}",
                            uid, e
                        );
                            break;
                        }
                    }
                }
            });

            let (write_packets, inner_read_packets): (Sender<SendPacket>, Receiver<SendPacket>) =
                unbounded();

            // Write packets
            let write_packet_handle = self.runtime.spawn(async move {
                while let Ok(packet) = inner_read_packets.recv() {
                    // Write
                    let mut packet = match bincode::serialize(&packet.0) {
                        Ok(val) => val,
                        Err(e) => {
                            error!("Error reading data from client {:?}: {:?}", uid, e);
                            break;
                        }
                    };

                    let mut data = Vec::new();

                    data.append(&mut bincode::serialize::<u32>(&(packet.len() as u32)).unwrap());
                    data.append(&mut packet);

                    if let Err(e) = write_tcp.write_all(&data).await {
                        warn!("Failed to write packet for user {:?}: {:?}", uid, e);
                        break;
                    }

                    if let Err(e) = write_tcp.flush().await {
                        warn!("Failed to flush packet for user {:?}: {:?}", uid, e);
                        break;
                    }
                }
            });

            self.users.insert(uid, NetworkUser {
                id: uid,
                read_packets,
                write_packets,
                read_packet_handle,
                write_packet_handle
            });

            connections.push(ConnectionEvent {
                user: uid
            });
        }

        connections
    }
}