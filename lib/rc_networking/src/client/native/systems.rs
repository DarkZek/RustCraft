use crate::bistream::BiStream;
use crate::protocol::clientbound::server_state::ServerState;
use crate::types::{ReceivePacket, SendPacket};
use crate::{get_channel, Channel, Protocol};
use bevy::app::AppExit;
use bevy::log::{info, warn};
use bevy::prelude::{EventReader, EventWriter, Res, ResMut};
use futures::FutureExt;
use rc_shared::constants::UserId;
use tokio::sync::mpsc::error::TryRecvError;
use crate::client::NetworkingClient;

pub fn update_system(
    mut client: ResMut<NetworkingClient>,
    mut packets: EventReader<ReceivePacket>
) {
    // Facilitate pending connection conversion
    if client.data.pending_connection.is_some() {
        if let Some(new_connection) = client.data.pending_connection.as_mut().unwrap().now_or_never() {
            if let Ok(connection) = new_connection {
                client.data.connection = Some(connection);
            } else {
                warn!("Connection failed");
            }
            client.data.pending_connection = None;
        }
    }

    // Detect errors and disconnect from connection
    if client.data.connection.is_some() {
        if let Err(TryRecvError::Empty) = client.data.connection.as_mut().unwrap().err_recv.try_recv() {
            // No events!
        } else {
            // Either the writer was disconnected, or an error was given. Either way it's disconnected
            client.data.connection = None;
            warn!("Disconnected from connection");
        }
    }

    for packet in packets.read() {
        if let Protocol::ServerState(ServerState::Disconnecting) = packet.0 {
            // Disconnect
            client.data.connection = None;
            warn!("Server shutting down");
        }
    }
}

/// Take packets from ECS EventReader and add it to Writer to write to stream in other thread
pub fn write_packets_system(
    client: Res<NetworkingClient>,
    mut to_send: EventReader<SendPacket>,
) {
    if to_send.len() == 0 {
        return;
    }

    let Some(conn) = &client.data.connection else {
        warn!(
            "Tried to send packet when disconnected {:?}",
            to_send.read().collect::<Vec<&SendPacket>>()
        );
        return;
    };

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
}

/// Detect shutdowns and close networking client
pub fn detect_shutdown_system(
    mut client: ResMut<NetworkingClient>,
    mut bevy_shutdown: EventReader<AppExit>,
) {
    for _ in bevy_shutdown.read() {
        info!("Shutting down connection");
        if let Some(mut connection) = client.data.connection.take() {
            connection
                .connection
                .close(0_u8.into(), "Closed");
        }
        if let Some(endpoint) = client.data.endpoint.take() {
            endpoint.close(0_u8.into(), "Closed".as_bytes());
        }
        if let Some(runtime) = client.data.runtime.take() {
            runtime.shutdown_background();
        }
    }
}

/// Take packets from Receivers and add it to ECS EventWriter
pub fn send_packets_system(
    mut client: ResMut<NetworkingClient>,
    mut recv: EventWriter<ReceivePacket>,
) {
    if let Some(conn) = &mut client.data.connection {
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
