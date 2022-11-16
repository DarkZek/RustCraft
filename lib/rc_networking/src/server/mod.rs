mod connection;
mod poll;
mod send;
mod user;

use crate::command::NetworkCommand;
use crate::error::NetworkingError;
use crate::server::poll::ConnectionRequest;
use crate::server::user::NetworkUser;
use bevy::log::{error, info};
use crossbeam::channel::{unbounded, Receiver, Sender};
use rc_protocol::constants::UserId;
use rc_protocol::protocol::serverbound::disconnect::Disconnect;
use rc_protocol::protocol::Protocol;
use rc_protocol::types::SendPacket;
use std::collections::HashMap;
use std::net::IpAddr;
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use bevy::ecs::prelude::Resource;

#[derive(Resource)]
pub struct ServerSocket {
    listen_address: IpAddr,
    port: usize,

    connected: bool,

    runtime: Runtime,

    send_commands: Sender<NetworkCommand>,

    receive_connections: Receiver<ConnectionRequest>,

    lifetime_connections: usize,

    users: HashMap<UserId, NetworkUser>,
}

impl ServerSocket {
    pub fn listen(ip: IpAddr, port: usize) -> Result<ServerSocket, NetworkingError> {
        info!("Listening for events on {}:{}", ip, port);

        // Start tokio thread
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .thread_name("ServerSocket")
            .build()
            .expect("Could not build tokio runtime");

        let (send_commands, receive_commands) = unbounded();

        let (send_connections, receive_connections) = unbounded();

        let listener = match runtime.block_on(TcpListener::bind(format!("{}:{}", ip, port))) {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to bind to port {}:{} {:?}", ip, port, e);
                return Err(NetworkingError::ConnectionRefused);
            }
        };

        // Spawn thread that listens for new clients
        runtime.spawn(async move {
            loop {
                // Read events
                if !receive_commands.is_empty() {
                    for event in receive_commands.iter() {
                        if let NetworkCommand::Stop = event {
                            return;
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
            connected: true,
            lifetime_connections: 0,
            users: Default::default(),
        })
    }

    pub fn shutdown(self) {
        // TODO: More gracefully
        self.runtime.shutdown_background();
    }

    pub fn send_command(&mut self, command: NetworkCommand) {
        match &command {
            NetworkCommand::Disconnect(uid) => {
                if let Some(user) = self.users.remove(uid) {
                    let _discard = user.write_packets.send(SendPacket(
                        Protocol::Disconnect(Disconnect::new(0)),
                        UserId(0),
                    ));
                }
            }
            NetworkCommand::Stop => {}
        }

        self.send_commands.send(command).unwrap();
    }
}
