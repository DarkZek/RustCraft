mod poll;
mod send;

use std::net::IpAddr;
use bevy_log::{debug, error, info, warn};
use crossbeam::channel::{Receiver, Sender, unbounded};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;
use rc_protocol::constants::UserId;
use rc_protocol::protocol::Protocol;
use rc_protocol::types::{ReceivePacket, SendPacket};
use crate::command::NetworkCommand;
use crate::error::NetworkingError;

pub struct ClientSocket {
    listen_address: IpAddr,
    port: usize,

    runtime: Runtime,

    send_commands: Sender<NetworkCommand>,

    read_packets: Receiver<ReceivePacket>,
    write_packets: Sender<SendPacket>,

    read_packet_handle: JoinHandle<()>,
    write_packet_handle: JoinHandle<()>
}

impl ClientSocket {
    pub fn connect(ip: IpAddr, port: usize) -> Result<ClientSocket, NetworkingError> {

        info!("Connecting to server on {}:{}", ip, port);

        // Start tokio thread
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .thread_name("ClientSocket")
            .build()
            .expect("Could not build tokio runtime");

        let (send_commands, receive_commands) = unbounded();

        let (inner_write_packets, read_packets) = unbounded();
        let (write_packets, inner_read_packets): (Sender<SendPacket>, Receiver<SendPacket>) = unbounded();

        let mut stream = match runtime.block_on(TcpStream::connect(format!("{}:{}", ip, port))) {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to bind to port {}:{} {:?}", ip, port, e);
                return Err(NetworkingError::ConnectionRefused)
            }
        };

        let (mut read_tcp, mut write_tcp) = stream.into_split();

        // Spawn thread that listens for new clients
        let read_packet_handle = runtime.spawn(async move {
            let mut len = [0; 4];
            while let Ok(size) = read_tcp.read_exact(&mut len).await {
                assert_eq!(size, 4);

                let len = bincode::deserialize::<u32>(&len).unwrap();

                // Collect data
                let mut data = vec![0; len as usize];

                if let Err(e) = read_tcp.read_exact(&mut data).await {
                    panic!("Grr {:?}", e);
                }

                // Turn it into packet
                let packet = bincode::deserialize::<Protocol>(&mut data).unwrap();
                debug!("-> {:?}", packet);

                inner_write_packets.send(ReceivePacket(packet, UserId(0))).unwrap();
            }
        });

        // Spawn thread that listens for new clients
        let write_packet_handle = runtime.spawn(async move {
            while let Ok(packet) = inner_read_packets.recv() {
                debug!("<- {:?}", packet.1);
                // Write
                let mut packet = match bincode::serialize(&packet.0) {
                    Ok(val) => val,
                    Err(e) => {
                        error!("Error reading data from server {:?}", e);
                        break;
                    }
                };

                let mut data = Vec::new();
                data.append(&mut bincode::serialize::<u32>(&(packet.len() as u32)).unwrap());
                data.append(&mut packet);

                if let Err(e) = write_tcp.write_all(&data).await {
                    warn!("Failed to write packet {:?}", e);
                    break;
                }

                if let Err(e) = write_tcp.flush().await {
                    warn!("Failed to flush packet for user {:?}", e);
                    break;
                }
            }
        });

        Ok(ClientSocket {
            listen_address: ip,
            port,
            runtime,
            send_commands,
            read_packets,
            write_packets,
            read_packet_handle,
            write_packet_handle
        })
    }

    pub fn shutdown(self) {
        // TODO: More gracefully
        self.runtime.shutdown_background();
    }
}