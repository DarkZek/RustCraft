use crate::bistream::BiStream;
use crate::client::NetworkingClient;
use crate::protocol::clientbound::server_state::ServerState;
use crate::types::{ReceivePacket, SendPacket};
use crate::{get_channel, Channel, Protocol};
use bevy::app::AppExit;
use bevy::log::{info, warn};
use bevy::prelude::{debug, EventReader, EventWriter, NonSendMut, ResMut, trace};
use futures::FutureExt;
use rc_shared::constants::UserId;
use tokio::sync::mpsc::error::TryRecvError;
use crate::client::wasm::server_connection::ServerConnection;

pub fn update_system(
    mut client: NonSendMut<NetworkingClient>,
    mut packets: EventReader<ReceivePacket>,
) {
    // Facilitate pending connection conversion
    if let Some(pending_connections_recv) = &mut client.pending_connections_recv {
        match pending_connections_recv.try_recv() {
            Ok(new_connection) => {
                if let Some(connection) = new_connection {
                    debug!("Connection established");
                    client.connection = Some(connection);
                } else {
                    warn!("Connection failed");
                }
                client.pending_connections_recv = None;
            }
            Err(e) => {
                match e {
                    TryRecvError::Empty => {}
                    TryRecvError::Disconnected => {
                        warn!("Pending connection failed");
                        client.pending_connections_recv = None;
                    }
                }
            }
        }
    }
}

/// Take packets from ECS EventReader and add it to Writer to write to stream in other thread
pub fn write_packets_system(
    mut client: NonSendMut<NetworkingClient>,
    mut to_send: EventReader<SendPacket>,
) {
    if to_send.len() == 0 {
        return;
    }
    if let Some(conn) = &mut client.connection {
        for packet in to_send.read() {
            let res = match get_channel(&packet.0) {
                Channel::Reliable => conn.reliable.send(packet.0.clone()),
                Channel::Unreliable => conn.unreliable.send(packet.0.clone()),
                Channel::Chunk => conn.chunk.send(packet.0.clone()),
            };

            if res.is_err() {
                // Connection closed
                warn!("Sending packets to writers errored: {:?}", res);
            }
        }
    } else {
        warn!(
            "Tried to send packet when disconnected {:?}",
            to_send.read().collect::<Vec<&SendPacket>>()
        );
    }
}

/// Detect shutdowns and close networking client
pub fn detect_shutdown_system(
    mut client: NonSendMut<NetworkingClient>,
    mut bevy_shutdown: EventReader<AppExit>,
) {
    for _ in bevy_shutdown.read() {
        info!("Shutting down server");
        if let Some(mut connection) = client.connection.take() {
            connection
                .connection
                .close(0_u8.into(), "Closed");
        }
    }
}

/// Take packets from Receivers and add it to ECS EventWriter
pub fn send_packets_system(
    mut client: NonSendMut<NetworkingClient>,
    mut recv: EventWriter<ReceivePacket>,
) {
    if let Some(conn) = &mut client.connection {
        let mut recieve_from_channel = |channel: &mut BiStream| {
            while let Ok(packet) = channel.try_recv() {
                recv.send(ReceivePacket(packet, UserId(0)));
            }
        };
        recieve_from_channel(&mut conn.unreliable);
        recieve_from_channel(&mut conn.reliable);
        recieve_from_channel(&mut conn.chunk);
    }
}
