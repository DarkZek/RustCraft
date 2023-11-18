use crate::bistream::BiStream;
use crate::client::NetworkingClient;
use crate::constants::UserId;
use crate::protocol::clientbound::server_state::ServerState;
use crate::types::{ReceivePacket, SendPacket};
use crate::{get_channel, Channel, Protocol};
use bevy::app::AppExit;
use bevy::prelude::{info, warn, EventReader, EventWriter, ResMut};
use futures::FutureExt;
use tokio::sync::mpsc::error::TryRecvError;

pub fn update_system(
    mut client: ResMut<NetworkingClient>,
    mut packets: EventReader<ReceivePacket>,
) {
    // Facilitate pending connection conversion
    if client.pending_connection.is_some() {
        if let Some(new_connection) = client.pending_connection.as_mut().unwrap().now_or_never() {
            if let Ok(connection) = new_connection {
                client.connection = Some(connection);
            } else {
                warn!("Connection failed");
            }
            client.pending_connection = None;
        }
    }

    // Detect errors and disconnect from server
    if client.connection.is_some() {
        if let Err(TryRecvError::Empty) = client.connection.as_mut().unwrap().err_recv.try_recv() {
            // No events!
        } else {
            // Either the writer was disconnected, or an error was given. Either way it's disconnected
            client.connection = None;
            warn!("Disconnected from server");
        }
    }

    for packet in packets.iter() {
        if let Protocol::ServerState(ServerState::Disconnecting) = packet.0 {
            // Disconnect
            client.connection = None;
            warn!("Server shutting down");
        }
    }
}

/// Take packets from ECS EventReader and add it to Writer to write to stream in other thread
pub fn write_packets_system(
    client: ResMut<NetworkingClient>,
    mut to_send: EventReader<SendPacket>,
) {
    if to_send.len() == 0 {
        return;
    }
    if let Some(conn) = &client.connection {
        for packet in to_send.iter() {
            let res = match get_channel(&packet.0) {
                Channel::Reliable => conn.reliable.out_send.send(packet.0.clone()),
                Channel::Unreliable => conn.unreliable.out_send.send(packet.0.clone()),
                Channel::Chunk => conn.chunk.out_send.send(packet.0.clone()),
            };

            if res.is_err() {
                // Connection closed
                warn!("Sending packets to writers errored: {:?}", res);
            }
        }
    } else {
        warn!(
            "Tried to send packet when disconnected {:?}",
            to_send.iter().collect::<Vec<&SendPacket>>()
        );
    }
}

/// Detect shutdowns and close networking client
pub fn detect_shutdown_system(
    mut client: ResMut<NetworkingClient>,
    mut bevy_shutdown: EventReader<AppExit>,
) {
    for _ in bevy_shutdown.iter() {
        info!("Shutting down server");
        if let Some(connection) = client.connection.take() {
            connection
                .connection
                .close(0_u8.into(), "Closed".as_bytes());
        }
        if let Some(endpoint) = client.endpoint.take() {
            endpoint.close(0_u8.into(), "Closed".as_bytes());
        }
        if let Some(runtime) = client.runtime.take() {
            runtime.shutdown_background();
        }
    }
}

/// Take packets from Receivers and add it to ECS EventWriter
pub fn send_packets_system(
    mut client: ResMut<NetworkingClient>,
    mut recv: EventWriter<ReceivePacket>,
) {
    if let Some(conn) = &mut client.connection {
        let mut recieve_from_channel = |channel: &mut BiStream| {
            while let Ok(packet) = channel.in_recv.try_recv() {
                recv.send(ReceivePacket(packet, UserId(0)));
            }
        };
        recieve_from_channel(&mut conn.unreliable);
        recieve_from_channel(&mut conn.reliable);
        recieve_from_channel(&mut conn.chunk);
    }
}
