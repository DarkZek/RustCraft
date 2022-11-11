use crate::transport::command::NetworkCommand;
use crate::transport::events::NetworkEvent;
use crate::ServerError;

use bevy_log::{error, info};
use crossbeam::channel::{unbounded, Receiver, Sender};

use std::net::IpAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Runtime;

pub struct ServerListener {
    pub runtime: Runtime,

    pub receive_events: Receiver<NetworkEvent>,
    pub send_commands: Sender<NetworkCommand>,

    pub receive_connections: Receiver<ConnectionRequest>,
}

impl ServerListener {
    pub fn new(ip: IpAddr, port: usize) -> Result<ServerListener, ServerError> {
        info!("Listening for events on {}:{}", ip, port);

        // Start tokio thread
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Could not build tokio runtime");

        let (_send_events, receive_events) = unbounded();
        let (send_commands, receive_commands) = unbounded();

        let (send_connections, receive_connections) = unbounded();

        // TODO: Move this to when we actually want to connect to a server
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

        Ok(ServerListener {
            receive_events,
            runtime,
            send_commands,
            receive_connections,
        })
    }
}

pub struct ConnectionRequest(pub TcpStream);
