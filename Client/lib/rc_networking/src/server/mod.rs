mod poll;
mod user;
mod connection;
mod send;

use std::collections::HashMap;
use std::net::IpAddr;
use bevy_log::{error, info};
use crossbeam::channel::{Receiver, Sender, unbounded};
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use rc_protocol::constants::UserId;
use crate::command::NetworkCommand;
use crate::error::NetworkingError;
use crate::server::poll::ConnectionRequest;
use crate::server::user::NetworkUser;

pub struct ServerSocket {
    pub listen_address: IpAddr,
    pub port: usize,

    pub connected: bool,

    pub runtime: Runtime,

    pub send_commands: Sender<NetworkCommand>,

    pub receive_connections: Receiver<ConnectionRequest>,

    pub lifetime_connections: usize,

    pub users: HashMap<UserId, NetworkUser>
}

impl ServerSocket {
    pub fn connect(ip: IpAddr, port: usize) -> Result<ServerSocket, NetworkingError> {

        info!("Listening for events on {}:{}", ip, port);

        // Start tokio thread
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Could not build tokio runtime");

        let (send_commands, receive_commands) = unbounded();

        let (send_connections, receive_connections) = unbounded();

        // Spawn thread that listens for new clients
        runtime.spawn(async move {
            let listener = match TcpListener::bind(format!("{}:{}", ip, port)).await {
                Ok(val) => val,
                Err(e) => {
                    error!("Failed to bind to port {}:{} {:?}", ip, port, e);
                    return;
                }
            };

            loop {
                // Read events
                if !receive_commands.is_empty() {
                    for event in receive_commands.iter() {
                        match event {
                            NetworkCommand::Disconnect => break,
                        }
                    }
                }

                // Try connect new clients
                let resp = match listener.accept().await {
                    Ok((socket, _)) => ConnectionRequest(socket),
                    Err(error) => {
                        error!("Error accepting new client {:?}", error);
                        continue;
                    }
                };

                match send_connections.send(resp) {
                    Ok(_) => {}
                    Err(err) => {
                        error!("Error sending connections {}", err);
                        break;
                    }
                }
            }
        });

        Ok(ServerSocket {
            listen_address: ip,
            port,
            runtime,
            send_commands,
            receive_connections,
            connected: false,
            lifetime_connections: 0,
            users: Default::default()
        })
    }
}